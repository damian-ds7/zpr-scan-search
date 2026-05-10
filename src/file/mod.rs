#[cfg(test)]
mod tests;
use crate::error::Result;
use crate::text_cacher::process_and_cache;
use crate::text_cacher::{CacheBackend, CachedDocument, FileFingerprint, LocalCache, WordMap};
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
    map: Arc<WordMap>,
}

impl TextFile {
    #[cfg(test)]
    pub fn new_raw(path: PathBuf, text: String, map: WordMap) -> Self {
        Self {
            path,
            text: Arc::new(text),
            map: Arc::new(map),
        }
    }

    /// Creates a new TextFile by either loading from cache or extracting from source using the provided extractor.
    pub fn new<E: TextExtractor>(path: PathBuf, extractor: &E) -> Result<TextFile> {
        let fp = FileFingerprint::from_path(&path)?;
        let backend = LocalCache::new();

        if let Ok(Some(CachedDocument { text, map, .. })) = backend.try_load(&path, &fp) {
            return Ok(Self {
                path,
                text: Arc::new(text),
                map: Arc::new(map),
            });
        }

        let text = extractor.extract_from(&path)?;
        let (text, map) = process_and_cache(text, path.clone(), fp, &backend);
        Ok(Self { path, text, map })
    }

    fn path(&self) -> &Path {
        &self.path
    }

    pub fn get(&self, key: &str) -> Option<&Vec<i32>> {
        self.map.get(key)
    }

    // TODO: can probably be removed after preview python function is not needed
    pub fn map(&self) -> &WordMap {
        &self.map
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}
