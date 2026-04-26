use crate::constants::DELIMITER;
use crate::error::Result;
use crate::file::TextFile;
use crate::ocr::OcrEngine;
use crate::text_extractor::{PdfExtractor, TextExtractor};
use image::DynamicImage;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

struct MockExtractor;
impl TextExtractor for MockExtractor {
    fn extract_from(&self, _path: &Path) -> Result<String> {
        Ok("mocked text content".to_string())
    }
}

struct MockOcr;
impl OcrEngine for MockOcr {
    fn extract_text_from_image(&self, _image_data: DynamicImage) -> Result<String> {
        Ok("mocked ocr text".to_string())
    }
}

#[test]
fn test_try_load_cache() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.pdf");
    let cache_path = dir.path().join("test.pdf.cache");

    let mut map = HashMap::new();
    map.insert("test".to_string(), vec![0]);
    let json_map = serde_json::to_string(&map).unwrap();

    let mut file = fs::File::create(&cache_path).unwrap();
    file.write_all(json_map.as_bytes()).unwrap();
    file.write_all(&[DELIMITER]).unwrap();
    file.write_all(b"test content").unwrap();

    let result = TextFile::try_load_cache(&file_path);
    assert!(result.is_ok());
    let (text, map_loaded) = result.unwrap();
    assert_eq!(text, "test content");
    assert_eq!(map_loaded.get("test").unwrap(), &vec![0]);
}

#[test]
fn test_new_with_mock_extractor() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.pdf");
    let extractor = MockExtractor;

    let text_file = TextFile::new(file_path, &extractor).unwrap();
    assert_eq!(*text_file.text, "mocked text content");
}
