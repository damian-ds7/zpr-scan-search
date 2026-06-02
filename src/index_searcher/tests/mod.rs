use super::IndexSearcher;
use crate::cacher::WordMap;
use crate::file::TextFile;
use crate::searcher::{Query, Search, SearchContext};
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
        String::from(content).into(),
        map.into(),
        None.into(),
    ))
}

#[test]
fn test_search_existing_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "quick brown fox".into(),
        ..Default::default()
    };
    let searcher = IndexSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    assert_eq!(iter.next().unwrap().matched(), "quick brown fox");
}

#[test]
fn test_search_non_existent_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "quick red fox".into(),
        ..Default::default()
    };
    let searcher = IndexSearcher::new(file);
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
    let searcher = IndexSearcher::new(file);
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
    let searcher = IndexSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    assert_eq!(iter.next().unwrap().matched(), "deep dark forest");
}

#[test]
fn test_search_repeated_phrase() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "jumps over the lazy dog".into(),
        ..Default::default()
    };
    let searcher = IndexSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    assert_eq!(iter.next().unwrap().matched(), "jumps over the lazy dog");
    assert_eq!(iter.next().unwrap().matched(), "jumps over the lazy dog");
}

#[test]
fn test_edge_case_rarest_at_beginning() {
    let text = "rarestword some some some";
    let file = create_test_file(text);
    let query = Query {
        term: "some rarestword".into(),
        ..Default::default()
    };
    let searcher = IndexSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_search_with_context_before() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "quick brown fox".into(),
        context: SearchContext {
            before: 1,
            after: 0,
        },
    };
    let searcher = IndexSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    let res = iter.next().unwrap();
    assert_eq!(res.matched(), "quick brown fox");
    assert_eq!(res.before(), "the");
    assert_eq!(res.after(), "");
}

#[test]
fn test_search_with_context_after() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "quick brown fox".into(),
        context: SearchContext {
            before: 0,
            after: 2,
        },
    };
    let searcher = IndexSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    let res = iter.next().unwrap();
    assert_eq!(res.matched(), "quick brown fox");
    assert_eq!(res.before(), "");
    assert_eq!(res.after(), "jumps over");
}

#[test]
fn test_search_with_context_before_and_after_text_edge() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "quick brown fox".into(),
        context: SearchContext {
            before: 3,
            after: 2,
        },
    };
    let searcher = IndexSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    let res = iter.next().unwrap();
    assert_eq!(res.matched(), "quick brown fox");
    assert_eq!(res.before(), "the");
    assert_eq!(res.after(), "jumps over");
}

#[test]
fn test_search_with_context_before_and_after() {
    let file = create_test_file(TEST_DOCUMENT);
    let query = Query {
        term: "forest filler filler".into(),
        context: SearchContext {
            before: 3,
            after: 2,
        },
    };
    let searcher = IndexSearcher::new(file);
    let mut iter = searcher.search(&query).unwrap();
    let res = iter.next().unwrap();
    assert_eq!(res.matched(), "forest filler filler");
    assert_eq!(res.before(), "into deep dark");
    assert_eq!(res.after(), "the quick");
}
