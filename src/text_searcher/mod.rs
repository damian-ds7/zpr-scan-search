use crate::file::TextFile;
use crate::searcher::{Search, SearchableIterator};
use std::str::SplitWhitespace;
#[cfg(test)]
pub mod tests;

struct TextSearcherIterator<'a> {
    iterator: SplitWhitespace<'a>,
    locations: Vec<i32>,
}
impl<'a> TextSearcherIterator<'a> {
    fn new(file: &'a TextFile, locations: Vec<i32>) -> Self {
        let iterator = file.text().split_whitespace();

        TextSearcherIterator {
            iterator,
            locations,
        }
    }
}

impl<'a> SearchableIterator<'a> for TextSearcherIterator<'a> {
    fn nth(&mut self, index: usize) -> Option<&'a str> {
        if index < self.locations.len() {
            let val = self.locations.iter().nth(index)?;
            Some(self.iterator.clone().nth(*val as usize)?)
        } else {
            None
        }
    }
}

struct TextSearcher<'a> {
    file: &'a TextFile,
}

impl<'a> TextSearcher<'a> {
    fn new(file: &'a TextFile) -> Self {
        TextSearcher { file }
    }
}
impl<'a> Search for TextSearcher<'a> {
    fn search(&self, query: &str) -> impl SearchableIterator<'_> {
        let words: Vec<&str> = query.split_whitespace().collect();
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
                let iterator = TextSearcherIterator::new(self.file, locations);
                return iterator;
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
        let iterator = TextSearcherIterator::new(self.file, locations);
        iterator
    }
}
