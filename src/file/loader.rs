use std::sync::Arc;

use crate::{
    error::Result,
    file::TextFile,
    supported_file::SupportedFile,
    text_cacher::{CacheBackend, CachedDocument, FileFingerprint, Job, codec::process_text},
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

    /// Loads a TextFile from the given `SupportedFile`.
    ///
    /// It first tries to load from the cache backend. If not found or stale, it uses the extractor
    /// and then saves the result to the cache.
    pub fn load(&self, file: SupportedFile) -> Result<TextFile> {
        let path = &file.path;
        let fp = FileFingerprint::from_path(path)?;

        if let Ok(Some(CachedDocument { text, map, .. })) = self.backend.try_load(path, &fp) {
            return Ok(TextFile {
                path: path.to_path_buf(),
                text: Arc::new(text),
                map: Arc::new(map),
                embeddings: None,
            });
        }
        let raw_text = self.extractor.extract_from(&file)?;

        let (text, map) = process_text(raw_text);
        let embeddings = Arc::new(None);
        self.backend.submit_job(
            path.clone(),
            Job::CacheWrite {
                text: Arc::clone(&text),
                map: Arc::clone(&map),
                fingerprint: fp,
                embeddings,
            },
        );

        Ok(TextFile {
            path: path.to_path_buf(),
            text,
            map,
            embeddings: None,
        })
    }
}
