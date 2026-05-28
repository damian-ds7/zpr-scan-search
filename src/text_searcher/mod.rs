use crate::error::Result;
use crate::file::TextFile;
use crate::searcher::{ArcStrSlice, Query, Search, SearchContext, SearchResult};
use crate::searcher_utils::{build_context_ranges, collect_context_fragments};
use std::sync::Arc;
#[cfg(test)]
pub mod tests;

/// SearchableIterator with search results being words in the file
struct TextSearcherIterator {
    text: Arc<str>,
    locations: Vec<i32>,
    pos: usize,
    word_pos: usize,
    byte_pos: usize,
    context: SearchContext,
}

impl TextSearcherIterator {
    fn new(text: Arc<str>, locations: Vec<i32>, context: SearchContext) -> Self {
        TextSearcherIterator {
            text: text.clone(),
            locations,
            context,
            pos: 0,
            word_pos: 0,
            byte_pos: 0,
        }
    }
}

impl Iterator for TextSearcherIterator {
    type Item = SearchResult;

    fn next(&mut self) -> Option<Self::Item> {
        let target_index = *self.locations.get(self.pos)? as usize;
        self.pos += 1;

        let before = self.context.before.unwrap_or(0);
        let after = self.context.after.unwrap_or(0);

        let fetch_from = target_index.saturating_sub(before);

        if self.word_pos > fetch_from {
            self.word_pos = 0;
            self.byte_pos = 0;
        }

        let words = collect_context_fragments(
            &self.text,
            self.text[self.byte_pos..].split_whitespace(),
            fetch_from,
            target_index + after,
            &mut self.word_pos,
            &mut self.byte_pos,
        )?;

        let mid = target_index - fetch_from;
        let (before_range, matched_range, after_range) = build_context_ranges(&words, mid);

        Some(SearchResult {
            before: ArcStrSlice::new(Arc::clone(&self.text), before_range),
            matched: ArcStrSlice::new(Arc::clone(&self.text), matched_range),
            after: ArcStrSlice::new(Arc::clone(&self.text), after_range),
        })
    }
}

pub(crate) struct TextSearcher {
    file: Arc<TextFile>,
}

/// A simple searcher looking for exact matches
impl TextSearcher {
    pub fn new(file: Arc<TextFile>) -> Self {
        TextSearcher { file }
    }
}

impl Search for TextSearcher {
    fn search(&self, query: &Query) -> Result<impl Iterator<Item = SearchResult>> {
        let words: Vec<&str> = query.term.split_whitespace().collect();
        let mut locations: Vec<i32> = vec![];

        let word_occur = match words
            .iter()
            .enumerate()
            .map(|(i, &word)| {
                self.file
                    .get(word)
                    .map(|occurrences| (i, word, occurrences))
            })
            .collect::<Option<Vec<(usize, &str, &Vec<i32>)>>>()
        {
            Some(mut valid_words) => {
                valid_words.sort_by_key(|w| w.2.len());
                valid_words
            }
            None => {
                let iterator = TextSearcherIterator::new(
                    self.file.text_arc(),
                    locations,
                    query.context.clone(),
                );
                return Ok(iterator);
            }
        };
        let rarest = word_occur[0];
        for location in rarest.2 {
            let location_usize = *location as usize;
            if location_usize >= rarest.0 {
                let is_match = word_occur.iter().all(|(word_index, _, occurrences)| {
                    let expected_pos = (location_usize - rarest.0 + word_index) as i32;
                    occurrences.binary_search(&expected_pos).is_ok()
                });
                if is_match {
                    locations.push(*location)
                }
            }
        }
        Ok(TextSearcherIterator::new(
            self.file.text_arc(),
            locations,
            query.context.clone(),
        ))
    }
}
