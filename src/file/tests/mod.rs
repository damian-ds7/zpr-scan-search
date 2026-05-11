use crate::error::Result;
use crate::file::TextFileLoader;
use crate::text_cacher::{CacheBackend, CachedDocument, FileFingerprint, Job, WordMap};
use crate::text_extractor::TextExtractor;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

struct MockExtractor;
impl TextExtractor for MockExtractor {
    fn extract_from(&self, _path: &Path) -> Result<String> {
        Ok("mocked text content".to_string())
    }
}

struct MockCache;
impl CacheBackend for MockCache {
    fn try_load(
        &self,
        path: &Path,
        fingerprint: &FileFingerprint,
    ) -> Result<Option<CachedDocument>> {
        let _ = path;

        Ok(Some(CachedDocument {
            text: "mocked text content".into(),
            map: WordMap::from("mocked text content"),
            fingerprint: fingerprint.clone(),
        }))
    }

    fn submit_job(&self, path: PathBuf, job: Job) {
        let _ = path;
        let _ = job;
    }
}

#[test]
fn test_loader_with_mock_extractor() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.pdf");
    let _file = fs::File::create(&file_path).unwrap();
    let extractor = MockExtractor;
    let backend = MockCache;
    let loader = TextFileLoader::new(extractor, backend);

    let text_file = loader.load(file_path).unwrap();
    assert_eq!(*text_file.text, "mocked text content");
}
