use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use walkdir::WalkDir;

use crate::{
    error::{Result, ScanSearchError},
    text_cacher::cache_writer::{Msg, WriteTask},
    text_cacher::{CacheBackend, CacheWriter},
};

use super::{CachedDocument, FileFingerprint, Job};

pub struct GlobalCache {
    pub path: PathBuf,
}

impl GlobalCache {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn get_cache_dir(&self, fingerprint: &FileFingerprint) -> PathBuf {
        self.path.join(format!(
            "{}-{}-{}",
            fingerprint.mtime_secs, fingerprint.mtime_nanos, fingerprint.size
        ))
    }

    fn get_file_name(path: &Path) -> Result<&str> {
        path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| ScanSearchError::NonUtf8Path(path.to_path_buf()))
    }

    fn get_text_path(dir: &Path, name: &str) -> PathBuf {
        dir.join(format!("{}.txt", name))
    }

    fn get_map_path(dir: &Path, name: &str) -> PathBuf {
        dir.join(format!("{}.map", name))
    }

    fn get_emb_path(dir: &Path, name: &str) -> PathBuf {
        dir.join(format!("{}.emb", name))
    }

    fn update_last_used(cache_dir: &Path) -> Result<()> {
        let last_used_path = cache_dir.join(".lastused");
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(std::io::Error::other)?
            .as_secs();

        CacheWriter::get().submit(Msg::Write(WriteTask {
            path: last_used_path,
            data: now.to_string().into_bytes(),
        }));
        Ok(())
    }

    fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
        let content = fs::read(path)?;
        Ok(serde_json::from_slice(&content)?)
    }

    fn submit_data(writer: &CacheWriter, path: PathBuf, data: Vec<u8>) {
        writer.submit(Msg::Write(WriteTask { path, data }));
    }

    fn submit_json<T: serde::Serialize>(
        writer: &CacheWriter,
        path: PathBuf,
        data: &T,
    ) -> serde_json::Result<()> {
        let data = serde_json::to_vec(data)?;
        Self::submit_data(writer, path, data);
        Ok(())
    }

    pub fn cleanup_unused(&self, days: u64) -> Result<()> {
        if !self.path.exists() {
            return Ok(());
        }

        let max_age_secs = days * 24 * 60 * 60;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(std::io::Error::other)?
            .as_secs();

        for entry in WalkDir::new(&self.path)
            .min_depth(2)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == ".lastused"
                && let Ok(content) = fs::read_to_string(entry.path())
                && let Ok(last_used) = content.trim().parse::<u64>()
                && now.saturating_sub(last_used) > max_age_secs
                && let Some(parent) = entry.path().parent()
            {
                let _ = fs::remove_dir_all(parent);
            }
        }
        Ok(())
    }
}

impl CacheBackend for GlobalCache {
    fn try_load(
        &self,
        path: &Path,
        fingerprint: &FileFingerprint,
        reload_cache: bool,
    ) -> Result<Option<CachedDocument>> {
        let cache_dir = self.get_cache_dir(fingerprint);
        if reload_cache || !cache_dir.exists() {
            return Ok(None);
        }

        let name = Self::get_file_name(path)?;
        let text_path = Self::get_text_path(&cache_dir, name);
        let map_path = Self::get_map_path(&cache_dir, name);
        let emb_path = Self::get_emb_path(&cache_dir, name);

        if !text_path.exists() || !map_path.exists() {
            return Ok(None);
        }

        let text = fs::read_to_string(text_path)?;
        let map = Self::read_json(&map_path)?;
        let embeddings = if emb_path.exists() {
            Some(Self::read_json(&emb_path)?)
        } else {
            None
        };

        let _ = Self::update_last_used(&cache_dir);

        Ok(Some(CachedDocument {
            text,
            map,
            fingerprint: fingerprint.clone(),
            embeddings,
        }))
    }

    fn submit_job(&self, path: PathBuf, job: Job) {
        let Job::CacheWrite {
            text,
            map,
            fingerprint,
            embeddings,
        } = job;

        let cache_dir = self.get_cache_dir(&fingerprint);
        if let Err(e) = fs::create_dir_all(&cache_dir) {
            eprintln!("Failed to create cache dir: {}", e);
            return;
        }

        let name = match Self::get_file_name(&path) {
            Ok(n) => n,
            Err(_) => return,
        };

        let writer = CacheWriter::get();

        Self::submit_data(
            writer,
            Self::get_text_path(&cache_dir, name),
            text.as_ref().as_bytes().to_vec(),
        );

        if let Err(e) =
            Self::submit_json(writer, Self::get_map_path(&cache_dir, name), map.as_ref())
        {
            eprintln!("Failed to serialize map: {}", e);
        }

        if let Some(emb) = embeddings.as_ref()
            && let Err(e) = Self::submit_json(writer, Self::get_emb_path(&cache_dir, name), emb)
        {
            eprintln!("Failed to serialize embeddings: {}", e);
        }

        let _ = Self::update_last_used(&cache_dir);
    }
}
