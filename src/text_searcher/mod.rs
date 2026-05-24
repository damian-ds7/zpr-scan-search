use crate::error::Result;
use crate::file::TextFile;
use crate::searcher::Search;
use std::sync::Arc;
#[cfg(test)]
pub mod tests;

/// SearchableIterator with search results being words in the file
struct TextSearcherIterator {
    file: Arc<TextFile>,
    locations: Vec<i32>,
    pos: usize,
}
impl TextSearcherIterator {
    fn new(file: Arc<TextFile>, locations: Vec<i32>) -> Self {
        TextSearcherIterator {
            file,
            locations,
            pos: 0,
        }
    }
}

impl Iterator for TextSearcherIterator {
    type Item = Arc<str>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = *self.locations.get(self.pos)? as usize;
        self.pos += 1;
        self.file.text().split_whitespace().nth(val).map(Arc::from)
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
    fn search(&self, query: &str) -> Result<impl Iterator<Item = Arc<str>>> {
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
                let iterator = TextSearcherIterator::new(self.file.clone(), locations);
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
        Ok(TextSearcherIterator::new(self.file.clone(), locations))
    }
}
