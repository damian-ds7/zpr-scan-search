use super::super::SemSearcher;
use crate::searcher::Search;
use crate::sem_searcher::tests::{
    LINE_FOREST, LINE_FOX_AND_DOG, LINE_JUMPS, MAIN_DOC, QUERY_JUMPS_OVER_LAZY_DOG,
    QUERY_QUICK_BROWN_FOX, create_test_file,
};
use crate::text_encoder::fastembed::FastEmbed;
use std::rc::Rc;

#[test]
fn searcher_ranks_lines_by_cosine_similarity() {
    let file = create_test_file(MAIN_DOC, &FastEmbed);
    let searcher = SemSearcher::new(file, FastEmbed, 10usize);
    let doc = MAIN_DOC.lines().collect::<Vec<_>>();

    let query = QUERY_QUICK_BROWN_FOX.to_string();
    let mut results = searcher.search(&query).unwrap();
    assert_eq!(results.next(), Some(Rc::from(doc[LINE_FOX_AND_DOG])));
    assert_eq!(results.next(), Some(Rc::from(doc[LINE_FOREST])));

    let query = QUERY_JUMPS_OVER_LAZY_DOG.to_string();
    let mut results = searcher.search(&query).unwrap();
    assert_eq!(results.next(), Some(Rc::from(doc[LINE_JUMPS])));
    assert_eq!(results.next(), Some(Rc::from(doc[LINE_FOX_AND_DOG])));
}
