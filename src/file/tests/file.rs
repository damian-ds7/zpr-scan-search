use crate::constants::DELIMITER;
use crate::file::TextFile;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_check_cache_loading() {
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

    let mut text_file = TextFile {
        path: file_path,
        text: String::new(),
        map: HashMap::new(),
    };

    let result = text_file.check_cache();
    assert!(result.is_ok());
    assert_eq!(text_file.text, "test content");
    assert_eq!(text_file.map.get("test").unwrap(), &vec![0]);
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_new_panics_due_to_unimplemented_read_pdf() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.pdf");
    let _ = TextFile::new(file_path);
}
