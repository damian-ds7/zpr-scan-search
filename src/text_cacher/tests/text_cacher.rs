use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use tempfile::tempdir;

use crate::text_cacher::cache_text;
use crate::file::file::TextFile;

const DELIMITER: &str = "\x1E";

#[test]
fn test_cache_text_multiple_cases() {
    let test_cases = [
        ("Ala ma kota", "{\"Ala\":[0],\"kota\":[2],\"ma\":[1]}"),
        (
            "Ala ma kota kota",
            "{\"Ala\":[0],\"kota\":[2,3],\"ma\":[1]}",
        ),
        ("hello hello hello", "{\"hello\":[0,1,2]}"),
        ("", "{}"),
    ];

    for (text, expected_json_str) in test_cases {
        let dir = tempdir().expect("Failed to create temp dir");
        let file_path = dir.path().join("my_document.pdf");

        let mut text_file_struct = TextFile::new_test(file_path.clone(), text.parse().unwrap()).expect("Failed to create TextFile");

        let cache_path = cache_text(text, &file_path, &mut text_file_struct).expect("Failed to cache text");
        let mut file = File::open(cache_path).expect("Failed to open cache file");
        let mut a = String::new();
        file.read_to_string(&mut a).expect("Failed to read");

        let (json_str, text_str) = a.split_once(DELIMITER).expect("Delimiter not found");
        let parsed_map: HashMap<String, Vec<i32>> =
            serde_json::from_str(json_str).expect("Failed to parse map");
        let expected_map: HashMap<String, Vec<i32>> =
            serde_json::from_str(expected_json_str).expect("Failed to parse expected map");

        assert_eq!(parsed_map, expected_map, "Failed on input: {}", text);
        assert_eq!(text_str, text, "Text mismatch on input: {}", text);
    }
}