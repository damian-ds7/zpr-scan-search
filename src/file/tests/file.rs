use crate::constants::DELIMITER;
use crate::error::Result;
use crate::file::TextFile;
use crate::ocr::OcrEngine;
use crate::text_cacher::{FileFingerprint, Job, execute_job};
use crate::text_extractor::TextExtractor;
use image::DynamicImage;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
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

    let fp = FileFingerprint::new_raw(1, 2, 3);

    let mut file = fs::File::create(&cache_path).unwrap();
    let job = Job::CacheWrite {
        text: Arc::new("test content".to_string()),
        map: Arc::new(map),
        fingerprint: fp.clone(),
        path: file_path.clone(),
    };
    let mut file = fs::File::create(&cache_path).unwrap();
    execute_job(&job, &mut file);

    let result = TextFile::try_load_cache(&file_path, &fp);
    assert!(result.is_ok());
    let option = result.unwrap();
    assert!(option.is_some());
    let cached_document = option.unwrap();
    assert_eq!(cached_document.text, "test content");
    assert_eq!(cached_document.map.get("test").unwrap(), &vec![0]);
    assert_eq!(cached_document.fingerprint, fp);
}

#[test]
fn test_new_with_mock_extractor() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.pdf");
    let file = fs::File::create(&file_path).unwrap();
    let extractor = MockExtractor;

    let text_file = TextFile::new(file_path, &extractor).unwrap();
    assert_eq!(*text_file.text, "mocked text content");
}
