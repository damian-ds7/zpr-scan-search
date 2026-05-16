mod fastembed_tests;

use super::SemSearcher;
use crate::error::Result;
use crate::file::TextFile;
use crate::searcher::{Search, SearchableIterator};
use crate::text_cacher::WordMap;
use crate::text_encoder::TextEncoder;
use std::path::PathBuf;

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

fn create_test_file(content: &str) -> TextFile {
    let mut map = WordMap::new();
    for (i, word) in content.split_whitespace().enumerate() {
        map.entry(word.to_string()).or_default().push(i as i32);
    }
    TextFile::new(PathBuf::from("test.txt"), content.to_string(), map)
}

struct MockEncoder;

impl TextEncoder for MockEncoder {
    fn encode(&self, text: &Vec<&str>) -> Result<Vec<Vec<f32>>> {
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
    let mut file = create_test_file(MAIN_DOC);
    let searcher = SemSearcher::new(&mut file, MockEncoder, 10usize);
    let doc = MAIN_DOC.lines().collect::<Vec<_>>();

    let query = QUERY_QUICK_BROWN_FOX.to_string();
    let mut results = searcher.search(&query);
    assert_eq!(results.get_at(0), Some(doc[LINE_FOX_AND_DOG]));
    assert_eq!(results.get_at(1), Some(doc[LINE_FOREST]));

    let query = QUERY_JUMPS_OVER_LAZY_DOG.to_string();
    let mut results = searcher.search(&query);
    assert_eq!(results.get_at(0), Some(doc[LINE_JUMPS]));
    assert_eq!(results.get_at(1), Some(doc[LINE_FOX_AND_DOG]));

    let query = QUERY_SOME_RARESTWORD.to_string();
    let mut results = searcher.search(&query);
    assert_eq!(results.get_at(0), Some(doc[LINE_FOX_AND_DOG]));
}

#[test]
fn searcher_returns_none_for_empty_query() {
    let mut file = create_test_file(MAIN_DOC);
    let searcher = SemSearcher::new(&mut file, MockEncoder, 10usize);
    let query = String::new();
    let mut results = searcher.search(&query);
    assert_eq!(results.get_at(0), None);
}
#[test]
fn searcher_returns_nothing_for_empty_doc() {
    let mut file = create_test_file("");
    let searcher = SemSearcher::new(&mut file, MockEncoder, 10usize);
    let query = QUERY_QUICK_BROWN_FOX.to_string();
    let mut results = searcher.search(&query);
    assert_eq!(results.get_at(0), None);
}
