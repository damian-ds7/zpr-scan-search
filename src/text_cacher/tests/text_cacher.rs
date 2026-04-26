use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use tempfile::tempdir;

use crate::text_cacher::{create_word_map, load_parts, process_and_cache};

#[test]
fn test_create_word_map_logic() {
    let test_cases = [
        (
            "Ala ma kota",
            vec![("Ala", vec![0]), ("ma", vec![1]), ("kota", vec![2])],
        ),
        ("kota kota", vec![("kota", vec![0, 1])]),
        ("", vec![]),
    ];

    for (text, expected) in test_cases {
        let map = create_word_map(text);
        assert_eq!(map.len(), expected.len());
        for (word, indices) in expected {
            assert_eq!(map.get(word).unwrap(), &indices);
        }
    }
}

#[test]
fn test_process_and_cache_async() {
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("document.pdf");
    let text = "hello world".to_string();

    let (returned_text, returned_map) = process_and_cache(text.clone(), file_path.clone());

    assert!(returned_map.contains_key("hello"));
    assert!(returned_map.contains_key("world"));

    let cache_path = dir.path().join("document.pdf.cache");
    let mut attempts = 0;
    while !cache_path.exists() && attempts < 50 {
        std::thread::sleep(std::time::Duration::from_millis(10));
        attempts += 1;
    }
    assert!(
        cache_path.exists(),
        "Cache file was not created in background"
    );

    let file = File::open(cache_path).expect("Failed to open cache");
    let mut reader = BufReader::new(file);
    let (loaded_text, loaded_map) = load_parts(&mut reader).expect("Failed to load parts");

    assert_eq!(*returned_map, loaded_map);
    assert_eq!(*returned_text, loaded_text);
}
