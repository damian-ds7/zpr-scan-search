use crate::file::TextFile;
use crate::text_searcher::search;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

const TEST_DOCUMENT: &str = "\
the quick brown fox jumps over the lazy dog and runs away \
the into deep dark forest filler filler the quick brown fox \
jumps over the lazy dog\
";

fn create_test_file(content: &str) -> TextFile {
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    let words: Vec<&str> = content.split_whitespace().collect();

    for (i, &word) in words.iter().enumerate() {
        map.entry(word.to_string()).or_default().push(i as i32);
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
    let locations = search(&file, &query);
    assert!(!locations.is_empty(), "Expected to find 'quick brown fox'");
}

#[test]
fn test_search_non_existent_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = "quick red fox".to_string();
    let locations = search(&file, &query);
    assert!(
        locations.is_empty(),
        "Expected empty vector for non-existent phrase"
    );
}

#[test]
fn test_search_rare_word_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = "deep dark forest".to_string();
    let locations = search(&file, &query);
    assert!(!locations.is_empty(), "Expected to find 'deep dark forest'");
}

#[test]
fn test_search_repeated_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = "jumps over the lazy dog".to_string();
    let locations = search(&file, &query);
    assert!(
        !locations.is_empty(),
        "Expected to find repeated phrase 'jumps over the lazy dog'"
    );
}

#[test]
fn test_edge_case_rarest_at_beginning() {
    let text = "rarestword some some some";
    let file = create_test_file(text);
    let query = "some rarestword".to_string();
    let locations = search(&file, &query);
    assert!(
        locations.is_empty(),
        "Expected empty vector because 'some rarestword' does not appear, checks the case when the rarestword is at the beginning"
    );
}
