use crate::error::Result;
use crate::file::TextFileLoader;
use crate::text_cacher::{
    CacheBackend, FileFingerprint, LocalCache, WordMap, serialize_cache_write,
};
use crate::text_extractor::TextExtractor;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tempfile::tempdir;

struct MockExtractor;
impl TextExtractor for MockExtractor {
    fn extract_from(&self, _path: &Path) -> Result<String> {
        Ok("mocked text content".to_string())
    }
}

#[test]
fn test_try_load_cache() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.pdf");
    let cache_path = dir.path().join("test.pdf.cache");

    let mut map = WordMap::new();
    map.insert("test".to_string(), vec![0]);

    let fp = FileFingerprint::new_raw(1, 2, 3);

    let text = Arc::new("test content".to_string());
    let map_arc = Arc::new(map);
    let mut file = fs::File::create(&cache_path).unwrap();
    serialize_cache_write(&text, &map_arc, &fp, &mut file).unwrap();

    let result = LocalCache::new().try_load(&file_path, &fp);
    assert!(result.is_ok());
    let option = result.unwrap();
    assert!(option.is_some());
    let cached_document = option.unwrap();
    assert_eq!(cached_document.text, "test content");
    assert_eq!(cached_document.map.get("test").unwrap(), &vec![0]);
    assert_eq!(cached_document.fingerprint, fp);
}

#[test]
fn test_loader_with_mock_extractor() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.pdf");
    let _file = fs::File::create(&file_path).unwrap();
    let extractor = MockExtractor;
    let backend = LocalCache::new();
    let loader = TextFileLoader::new(extractor, backend);

    let text_file = loader.load(file_path).unwrap();
    assert_eq!(*text_file.text, "mocked text content");
}
