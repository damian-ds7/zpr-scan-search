use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use crate::file::TextFile;
use crate::text_searcher::search;

const TEST_DOCUMENT: &str = "\
the quick brown fox jumps over the lazy dog and runs away \
the into deep dark forest filler filler the quick brown fox \
jumps over the lazy dog\
";

fn create_test_file(content: &str) -> TextFile {
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    let words: Vec<&str> = content.split_whitespace().collect();
    
    for (i, &word) in words.iter().enumerate() {
        map.entry(word.to_string())
            .or_default()
            .push(i as i32);
    }

    TextFile {
        path: PathBuf::from("test.txt"),
        text: Arc::new(content.to_string()),
        map: Arc::new(map),
    }
}

#[test]
fn test_search_existing_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = "quick brown fox".to_string();
    let result = search(&file, &query);
    assert!(result.is_ok());
}

#[test]
fn test_search_non_existent_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = "quick red fox".to_string();
    let result = search(&file, &query);
    assert!(result.is_err());
}

#[test]
fn test_search_rare_word_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = "deep dark forest".to_string();
    let result = search(&file, &query);
    assert!(result.is_ok());
}

#[test]
fn test_search_repeated_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = "jumps over the lazy dog".to_string();
    let result = search(&file, &query);
    assert!(result.is_ok());
}
