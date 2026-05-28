use super::super::SemSearcher;
use crate::searcher::{Query, Search};
use crate::sem_searcher::tests::{
    LINE_FOREST, LINE_FOX_AND_DOG, LINE_JUMPS, MAIN_DOC, QUERY_JUMPS_OVER_LAZY_DOG,
    QUERY_QUICK_BROWN_FOX, create_test_file,
};
use crate::text_encoder::fastembed::FastEmbed;

#[test]
fn searcher_ranks_lines_by_cosine_similarity() {
    let file = create_test_file(MAIN_DOC, &FastEmbed);
    let searcher = SemSearcher::new(file, FastEmbed, 10usize);
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
}
