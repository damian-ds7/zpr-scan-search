#[cfg(test)]
mod tests;
use crate::error::Result;
use crate::text_cacher::{CachedDocument, FileFingerprint};
use crate::text_cacher::{load_parts, process_and_cache};
use crate::text_extractor::TextExtractor;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Represents a processed document containing its text content and a word occurrence map.
pub struct TextFile {
    path: PathBuf,
    text: Arc<String>,
    map: Arc<HashMap<String, Vec<i32>>>,
}

impl TextFile {
    #[cfg(test)]
    pub fn new_raw(path: PathBuf, text: String, map: HashMap<String, Vec<i32>>) -> Self {
        Self {
            path,
            text: Arc::new(text),
            map: Arc::new(map),
        }
    }

    /// Creates a new TextFile by either loading from cache or extracting from source using the provided extractor.
    pub fn new<E: TextExtractor>(path: PathBuf, extractor: &E) -> Result<TextFile> {
        let fp = FileFingerprint::from_path(&path)?;

        if let Some(CachedDocument { text, map, .. }) = Self::try_load_cache(&path, &fp)? {
            return Ok(Self {
                path,
                text: Arc::new(text),
                map: Arc::new(map),
            });
        }

        let text = extractor.extract_from(&path)?;
        let (text, map) = process_and_cache(text, path.clone());
        Ok(Self { path, text, map })
    }

    /// Attempts to load the document content and word map from a local .cache file.
    fn try_load_cache(
        path: &Path,
        fingerprint: &FileFingerprint,
    ) -> Result<Option<CachedDocument>> {
        let mut cache_path = path.to_path_buf();

        if let Some(file_name) = cache_path.file_name().and_then(|f| f.to_str()) {
            cache_path.set_file_name(format!("{}.cache", file_name));
        }

        let file = File::open(cache_path.as_path())?;
        let mut reader = BufReader::new(file);
        let cached_file = load_parts(&mut reader)?;

        if cached_file.fingerprint != *fingerprint {
            return Ok(None);
        }

        Ok(Some(cached_file))
    }

    fn path(&self) -> &Path {
        &self.path
    }

    pub fn get(&self, key: &str) -> Option<&Vec<i32>> {
        self.map.get(key)
    }

    // TODO: can probably be removed after preview python function is not needed
    pub fn map(&self) -> &HashMap<String, Vec<i32>> {
        &self.map
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}
