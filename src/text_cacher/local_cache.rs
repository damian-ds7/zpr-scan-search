use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::text_cacher::cache_writer::{Msg, WriteTask};
use crate::text_cacher::{
    CacheBackend, CacheWriter, CachedDocument, FileFingerprint, Job, load_parts,
};

#[derive(Default)]
pub struct LocalCache;

impl CacheBackend for LocalCache {
    fn try_load(
        &self,
        path: &Path,
        fingerprint: &FileFingerprint,
    ) -> Result<Option<CachedDocument>> {
        let mut cache_path = path.to_path_buf();

        if let Some(file_name) = cache_path.file_name().and_then(|f| f.to_str()) {
            cache_path.set_file_name(format!("{}.cache", file_name));
        }

        let file = match File::open(cache_path.as_path()) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let mut reader = BufReader::new(file);
        let cached_file = load_parts(&mut reader)?;

        if cached_file.fingerprint != *fingerprint {
            return Ok(None);
        }

        Ok(Some(cached_file))
    }

    fn submit_job(&self, path: PathBuf, job: Job) {
        let mut cache_path = path;
        if let Some(file_name) = cache_path.file_name().and_then(|f| f.to_str()) {
            cache_path.set_file_name(format!("{}.cache", file_name));
        }

        match job {
            Job::CacheWrite {
                text,
                map,
                fingerprint,
            } => {
                let mut data = Vec::new();
                if let Err(e) =
                    crate::text_cacher::serialize_cache_write(&text, &map, &fingerprint, &mut data)
                {
                    eprintln!("Failed to serialize cache data for {:?}: {}", cache_path, e);
                    return;
                }

                CacheWriter::get().submit(Msg::Write(WriteTask {
                    data,
                    path: cache_path,
                }));
            }
        }
    }
}
