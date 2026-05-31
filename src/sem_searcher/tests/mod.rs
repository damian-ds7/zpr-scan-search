mod fastembed_tests;

use super::SemSearcher;
use crate::cacher::{Embeddings, WordMap};
use crate::encoder::TextEncoder;
use crate::error::Result;
use crate::file::TextFile;
use crate::searcher::{Query, Search};
use std::path::PathBuf;
use std::sync::Arc;

const MAIN_DOC: &str = "\
the quick brown fox jumps over the lazy dog and runs away
the into deep dark forest filler filler the quick brown fox
jumps over the lazy dog
filler 1
filler 2
filler 3
filler 4
filler 5
filler 6
filler 7
filler 8
filler 9
filler 10
";

const LINE_FOX_AND_DOG: usize = 0;
const LINE_FOREST: usize = 1;
const LINE_JUMPS: usize = 2;

const QUERY_QUICK_BROWN_FOX: &str = "quick brown fox";
const QUERY_JUMPS_OVER_LAZY_DOG: &str = "jumps over the lazy dog";
const QUERY_SOME_RARESTWORD: &str = "some rarestword";

pub(crate) fn create_test_file<E: TextEncoder>(content: &str, encoder: &E) -> Arc<TextFile> {
    let mut map = WordMap::new();
    for (i, word) in content.split_whitespace().enumerate() {
        map.entry(word.to_string()).or_default().push(i as i32);
    }
    let lines: Vec<&str> = content.lines().collect();
    let embeddings = encoder.encode(&lines).ok();
    Arc::new(TextFile::new(
        PathBuf::from("test.txt"),
        content.to_string(),
        map,
        embeddings,
    ))
}

struct MockEncoder;

impl TextEncoder for MockEncoder {
    fn encode(&self, text: &[&str]) -> Result<Embeddings> {
        let main_lines = MAIN_DOC.lines().collect::<Vec<_>>();

        text.iter()
            .map(|&s| match s {
                QUERY_QUICK_BROWN_FOX => vec![1.0, 0.0, 0.0, 0.0],
                QUERY_JUMPS_OVER_LAZY_DOG => vec![0.0, 1.0, 0.0, 0.0],
                QUERY_SOME_RARESTWORD => vec![1.0, 0.0, 0.0, 0.0],
                s if s == main_lines[LINE_FOX_AND_DOG] => vec![0.8, 0.6, 0.0, 0.0],
                s if s == main_lines[LINE_FOREST] => vec![0.6, 0.0, 0.8, 0.0],
                s if s == main_lines[LINE_JUMPS] => vec![0.0, 1.0, 0.0, 0.0],
                _ => vec![0.0, 0.0, 0.0, 1.0],
            })
            .map(Ok)
            .collect()
    }
}

#[test]
fn searcher_ranks_lines_by_cosine_similarity() {
    let file = create_test_file(MAIN_DOC, &MockEncoder);
    let searcher = SemSearcher::new(file, MockEncoder, 10usize);
    let doc = MAIN_DOC.lines().collect::<Vec<_>>();

    let query = Query {
        term: QUERY_QUICK_BROWN_FOX.into(),
        ..Default::default()
    };
    let mut results = searcher.search(&query).unwrap();
    assert_eq!(results.next().unwrap().matched(), doc[LINE_FOX_AND_DOG]);
    assert_eq!(results.next().unwrap().matched(), doc[LINE_FOREST]);

    let query = Query {
        term: QUERY_JUMPS_OVER_LAZY_DOG.into(),
        ..Default::default()
    };
    let mut results = searcher.search(&query).unwrap();
    assert_eq!(results.next().unwrap().matched(), doc[LINE_JUMPS]);
    assert_eq!(results.next().unwrap().matched(), doc[LINE_FOX_AND_DOG]);

    let query = Query {
        term: QUERY_SOME_RARESTWORD.into(),
        ..Default::default()
    };
    let mut results = searcher.search(&query).unwrap();
    assert_eq!(results.next().unwrap().matched(), doc[LINE_FOX_AND_DOG]);
}

#[test]
fn searcher_returns_none_for_empty_query() {
    let file = create_test_file(MAIN_DOC, &MockEncoder);
    let searcher = SemSearcher::new(file, MockEncoder, 10usize);
    let query = Query {
        term: "".into(),
        ..Default::default()
    };
    let mut results = searcher.search(&query).unwrap();
    assert_eq!(results.next(), None);
}
#[test]
fn searcher_returns_nothing_for_empty_doc() {
    let file = create_test_file("", &MockEncoder);
    let searcher = SemSearcher::new(file, MockEncoder, 10usize);
    let query = Query {
        term: QUERY_QUICK_BROWN_FOX.into(),
        ..Default::default()
    };
    let mut results = searcher.search(&query).unwrap();
    assert_eq!(results.next(), None);
}
