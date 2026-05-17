use crate::error::Result;
use crate::file::TextFileLoader;
use crate::supported_file::{FileKind, SupportedFile};
use crate::text_cacher::{CacheBackend, CachedDocument, FileFingerprint, Job, WordMap};
use crate::text_extractor::TextExtractor;
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
    let loader = TextFileLoader::new(extractor, backend);

    let file = SupportedFile {
        path: file_path,
        kind: FileKind::Pdf,
    };

    let text_file = loader.load(file).unwrap();

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
    let loader = TextFileLoader::new(extractor, backend);

    let file = SupportedFile {
        path: file_path,
        kind: FileKind::Pdf,
    };

    let text_file = loader.load(file).unwrap();

    assert_eq!(text_file.text(), "extracted text");
    assert!(*submit_called.lock().unwrap());
}
