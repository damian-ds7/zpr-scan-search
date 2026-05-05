use crate::file::TextFile;
use std::collections::HashMap;
use std::ops::Index;

#[cfg(test)]
pub mod tests;

pub fn search(file: &TextFile, query: &str) -> Vec<i32> {
    let words: Vec<&str> = query.split_whitespace().collect();
    let mut locations: Vec<i32> = vec![];

    let word_occur = match words
        .iter()
        .enumerate()
        .map(|(i, &word)| file.map.get(word).map(|occurrences| (i, word, occurrences)))
        .collect::<Option<Vec<(usize, &str, &Vec<i32>)>>>()
    {
        Some(mut valid_words) => {
            valid_words.sort_by_key(|w| w.2.len());
            valid_words
        }
        None => return locations,
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

    locations
}
