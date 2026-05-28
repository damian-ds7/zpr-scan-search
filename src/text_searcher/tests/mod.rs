use super::TextSearcher;
use crate::file::TextFile;
use crate::searcher::{Query, Search};
use crate::text_cacher::WordMap;
use std::path::PathBuf;
use std::sync::Arc;

const TEST_DOCUMENT: &str = "\
the quick brown fox jumps over the lazy dog and runs away \
the into deep dark forest filler filler the quick brown fox \
jumps over the lazy dog\
";

fn create_test_file(content: &str) -> Arc<TextFile> {
    let mut map = WordMap::new();
    let words: Vec<&str> = content.split_whitespace().collect();

    for (i, &word) in words.iter().enumerate() {
        map.entry(word.to_string()).or_default().push(i as i32);
    }

    Arc::new(TextFile::new(
        PathBuf::from("test.txt"),
        String::from(content),
        map,
        None,
    ))
}

#[test]
fn test_search_existing_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "quick brown fox".into(),
        ..Default::default()
    };
    let searcher = TextSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    assert_eq!(iter.next().as_deref(), Some("quick"));
}

#[test]
fn test_search_non_existent_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "quick red fox".into(),
        ..Default::default()
    };
    let searcher = TextSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_search_non_existent_phrase_with_existing_words() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "filler filler forest".into(),
        ..Default::default()
    };
    let searcher = TextSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_search_rare_word_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "deep dark forest".into(),
        ..Default::default()
    };
    let searcher = TextSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    assert_eq!(iter.next().as_deref(), Some("deep"));
}

#[test]
fn test_search_repeated_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "jumps over the lazy dog".into(),
        ..Default::default()
    };
    let searcher = TextSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    assert_eq!(iter.next().as_deref(), Some("jumps"));
    assert_eq!(iter.next().as_deref(), Some("jumps"));
}

#[test]
fn test_edge_case_rarest_at_beginning() {
    let text = "rarestword some some some";
    let file = create_test_file(text);
    let query = Query {
        term: "some rarestword".into(),
        ..Default::default()
    };
    let searcher = TextSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    assert_eq!(iter.next(), None);
}
