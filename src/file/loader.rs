use std::sync::Arc;

use crate::{
    cacher::{CacheBackend, CachedDocument, FileFingerprint, Job, process_text},
    encoder::TextEncoder,
    error::Result,
    extractor::TextExtractor,
    file::TextFile,
    supported_file::SupportedFile,
};

pub trait FileLoader: Sync + Send {
    fn load(&self, file: SupportedFile, embed: bool, reload_cache: bool) -> Result<TextFile>;
}

/// A loader that handles the process of loading a TextFile, either from cache or by extracting text.
///
/// The TextFile is purely a dataclass and the loader handles creating the TextFile.
/// It a TextExtractor, for extracting text from the pdf, a Cache Backend for processing cache
/// and a Text Encoder for generating embeddings used later by Semantic Search
pub struct TextFileLoader<E: TextExtractor, B: CacheBackend, C: TextEncoder> {
    extractor: E,
    backend: B,
    encoder: C,
}

impl<E: TextExtractor, B: CacheBackend, C: TextEncoder> TextFileLoader<E, B, C> {
    /// Creates a new TextFileLoader with the given extractor and cache backend.
    pub fn new(extractor: E, backend: B, encoder: C) -> Self {
        Self {
            extractor,
            backend,
            encoder,
        }
    }
}

impl<E: TextExtractor, B: CacheBackend, C: TextEncoder> FileLoader for TextFileLoader<E, B, C> {
    /// Loads a TextFile from the given `SupportedFile`.
    ///
    /// It first tries to load from the cache backend. If not found or stale, it uses the extractor
    /// and then saves the result to the cache.
    fn load(&self, file: SupportedFile, embed: bool, reload_cache: bool) -> Result<TextFile> {
        let path = &file.path;
        let fp = FileFingerprint::from_path(path)?;
        if let Ok(Some(CachedDocument {
            text,
            map,
            embeddings,
            ..
        })) = self.backend.try_load(path, &fp, reload_cache)
        {
            let text: Arc<str> = text.into_boxed_str().into();
            let map = Arc::new(map);
            let embeddings = if embed && embeddings.is_none() {
                let lines: Vec<&str> = text.lines().collect();
                let embeddings = Arc::new(Some(self.encoder.encode(&lines)?));
                self.backend.submit_job(
                    path.clone(),
                    Job::CacheWrite {
                        text: Arc::clone(&text),
                        map: Arc::clone(&map),
                        fingerprint: fp,
                        embeddings: Arc::clone(&embeddings),
                    },
                );
                embeddings
            } else {
                Arc::new(embeddings)
            };
            return Ok(TextFile {
                path: path.into(),
                text,
                map,
                embeddings,
            });
        }

        let raw_text = self.extractor.extract_from(&file)?;
        let (text, map) = process_text(raw_text);
        let embeddings = if embed {
            let lines: Vec<&str> = text.lines().collect();
            Arc::new(Some(self.encoder.encode(&lines)?))
        } else {
            Arc::new(None)
        };
        self.backend.submit_job(
            path.clone(),
            Job::CacheWrite {
                text: Arc::clone(&text),
                map: Arc::clone(&map),
                fingerprint: fp,
                embeddings: Arc::clone(&embeddings),
            },
        );
        Ok(TextFile {
            path: path.into(),
            text,
            map,
            embeddings,
        })
    }
}
