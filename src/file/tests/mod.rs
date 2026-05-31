use crate::cacher::{CacheBackend, CachedDocument, Embeddings, FileFingerprint, Job, WordMap};
use crate::encoder::TextEncoder;
use crate::encoder::fastembed::FastEmbed;
use crate::error::Result;
use crate::extractor::TextExtractor;
use crate::file::{FileLoader, TextFileLoader};
use crate::supported_file::{FileKind, SupportedFile};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

use std::sync::Arc;
use std::sync::Mutex;

struct MockExtractor;
impl TextExtractor for MockExtractor {
    fn extract_from(&self, _file: &SupportedFile) -> Result<String> {
        Ok("extracted text".to_string())
    }
}

struct SpyCache {
    should_hit: bool,
    submit_called: Arc<Mutex<bool>>,
}

impl SpyCache {
    fn new(should_hit: bool) -> Self {
        Self {
            should_hit,
            submit_called: Arc::new(Mutex::new(false)),
        }
    }
}

impl CacheBackend for SpyCache {
    fn try_load(
        &self,
        _path: &Path,
        fingerprint: &FileFingerprint,
    ) -> Result<Option<CachedDocument>> {
        if self.should_hit {
            Ok(Some(CachedDocument {
                text: "cached text".into(),
                map: WordMap::from("cached text"),
                fingerprint: fingerprint.clone(),
                embeddings: None,
            }))
        } else {
            Ok(None)
        }
    }

    fn submit_job(&self, _path: PathBuf, _job: Job) {
        let mut called = self.submit_called.lock().unwrap();
        *called = true;
    }
}

#[test]
fn test_loader_cache_hit() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.pdf");
    let _file = fs::File::create(&file_path).unwrap();

    let extractor = MockExtractor;
    let backend = SpyCache::new(true);
    let submit_called = backend.submit_called.clone();
    let encoder = FastEmbed::default();
    let loader = TextFileLoader::new(extractor, backend, encoder);

    let file = SupportedFile {
        path: file_path,
        kind: FileKind::Pdf,
    };

    let text_file = loader.load(file, false).unwrap();

    assert_eq!(text_file.text(), "cached text");
    assert!(!*submit_called.lock().unwrap());
}

#[test]
fn test_loader_cache_miss_triggers_extraction_and_cache() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.pdf");
    let _file = fs::File::create(&file_path).unwrap();

    let extractor = MockExtractor;
    let backend = SpyCache::new(false);
    let submit_called = backend.submit_called.clone();
    let encoder = FastEmbed::default();
    let loader = TextFileLoader::new(extractor, backend, encoder);

    let file = SupportedFile {
        path: file_path,
        kind: FileKind::Pdf,
    };

    let text_file = loader.load(file, false).unwrap();

    assert_eq!(text_file.text(), "extracted text");
    assert!(*submit_called.lock().unwrap());
}

struct MockEncoder;
impl TextEncoder for MockEncoder {
    fn encode(&self, _text: &[&str]) -> Result<Embeddings> {
        Ok(Embeddings::from(vec![vec![1.0, 2.0]]))
    }
}

#[derive(Clone)]
struct InMemoryCache {
    data: Arc<Mutex<Option<CachedDocument>>>,
}

impl InMemoryCache {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(None)),
        }
    }
}

impl CacheBackend for InMemoryCache {
    fn try_load(
        &self,
        _path: &Path,
        _fingerprint: &FileFingerprint,
    ) -> Result<Option<CachedDocument>> {
        let data = self.data.lock().unwrap();
        Ok(data.clone())
    }

    fn submit_job(&self, _path: PathBuf, job: Job) {
        let Job::CacheWrite {
            text,
            map,
            fingerprint,
            embeddings,
        } = job;

        let mut data = self.data.lock().unwrap();
        *data = Some(CachedDocument {
            text: text.to_string(),
            map: (*map).clone(),
            fingerprint,
            embeddings: (*embeddings).clone(),
        });
    }
}

#[test]
fn test_loader_recreates_embeddings_if_missing() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.pdf");
    let _file = fs::File::create(&file_path).unwrap();

    let extractor = MockExtractor;
    let backend = InMemoryCache::new();
    let encoder = MockEncoder;
    let loader = TextFileLoader::new(extractor, backend.clone(), encoder);

    let file = SupportedFile {
        path: file_path,
        kind: FileKind::Pdf,
    };

    let text_file = loader.load(file.clone(), false).unwrap();
    assert!(text_file.embeddings.is_none());

    let text_file_with_embeddings = loader.load(file, true).unwrap();
    assert!(text_file_with_embeddings.embeddings.is_some());

    let fp = FileFingerprint::from_path(&text_file_with_embeddings.path).unwrap();
    let cached = backend
        .try_load(&text_file_with_embeddings.path, &fp)
        .unwrap()
        .unwrap();
    assert!(cached.embeddings.is_some());
}
