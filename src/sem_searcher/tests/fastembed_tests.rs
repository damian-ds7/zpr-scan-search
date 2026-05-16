use super::super::SemSearcher;
use crate::error::Result;
use crate::file::TextFile;
use crate::searcher::{Search, SearchableIterator};
use crate::text_cacher::WordMap;
use crate::text_encoder::TextEncoder;
use std::path::PathBuf;
use crate::text_encoder::fastembed::FastEmbed;

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
const LINE_FOREST:      usize = 1;
const LINE_JUMPS:       usize = 2;

const QUERY_QUICK_BROWN_FOX:     &str = "quick brown fox";
const QUERY_JUMPS_OVER_LAZY_DOG: &str = "jumps over the lazy dog";

fn create_test_file(content: &str) -> TextFile {
    let mut map = WordMap::new();
    for (i, word) in content.split_whitespace().enumerate() {
        map.entry(word.to_string()).or_default().push(i as i32);
    }
    TextFile::new(PathBuf::from("test.txt"), content.to_string(), map)
}

#[test]
fn searcher_ranks_lines_by_cosine_similarity() {
    let mut file = create_test_file(MAIN_DOC);
    let searcher = SemSearcher::new(&mut file, FastEmbed{}, 10usize);
    let doc = MAIN_DOC.lines().collect::<Vec<_>>();

    let query = QUERY_QUICK_BROWN_FOX.to_string();
    let mut results = searcher.search(&query);
    assert_eq!(results.get_at(0), Some(doc[LINE_FOX_AND_DOG]));
    assert_eq!(results.get_at(1), Some(doc[LINE_FOREST]));

    let query = QUERY_JUMPS_OVER_LAZY_DOG.to_string();
    let mut results = searcher.search(&query);
    assert_eq!(results.get_at(0), Some(doc[LINE_JUMPS]));
    assert_eq!(results.get_at(1), Some(doc[LINE_FOX_AND_DOG]));
}
