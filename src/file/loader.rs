use std::{path::PathBuf, sync::Arc};

use crate::{
    error::Result,
    file::TextFile,
    text_cacher::{CacheBackend, CachedDocument, FileFingerprint, process_and_cache},
    text_extractor::TextExtractor,
};

/// A loader that handles the process of loading a TextFile, either from cache or by extracting text.
pub struct TextFileLoader<E: TextExtractor, B: CacheBackend> {
    extractor: E,
    backend: B,
}

impl<E: TextExtractor, B: CacheBackend> TextFileLoader<E, B> {
    /// Creates a new TextFileLoader with the given extractor and cache backend.
    pub fn new(extractor: E, backend: B) -> Self {
        Self { extractor, backend }
    }

    /// Loads a TextFile from the given path.
    /// It first tries to load from the cache backend. If not found or stale, it uses the extractor
    /// and then saves the result to the cache.
    pub fn load(&self, path: PathBuf) -> Result<TextFile> {
        let fp = FileFingerprint::from_path(&path)?;

        if let Ok(Some(CachedDocument { text, map, .. })) = self.backend.try_load(&path, &fp) {
            return Ok(TextFile {
                path,
                text: Arc::new(text),
                map: Arc::new(map),
            });
        }

        let text = self.extractor.extract_from(&path)?;
        let (text, map) = process_and_cache(text, path.clone(), fp, &self.backend);
        Ok(TextFile { path, text, map })
    }
}
