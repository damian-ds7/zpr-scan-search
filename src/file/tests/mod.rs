use crate::error::Result;
use crate::file::TextFileLoader;
use crate::text_cacher::LocalCache;
use crate::text_extractor::TextExtractor;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

struct MockExtractor;
impl TextExtractor for MockExtractor {
    fn extract_from(&self, _path: &Path) -> Result<String> {
        Ok("mocked text content".to_string())
    }
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
