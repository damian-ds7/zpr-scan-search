use crate::file::TextFile;
use std::collections::HashMap;

#[cfg(test)]
pub mod tests;

pub fn search(file: &TextFile, query: &String) -> Vec<i32> {
    let words: Vec<&str> = query.split_whitespace().collect();
    let mut locations: Vec<i32> = vec![];

    let word_occur: Option<Vec<(&str, &Vec<i32>)>> = words
        .clone()
        .into_iter()
        .map(|word| file.map.get(word).map(|occurances| (word, occurances)))
        .collect();
    let Some(mut valid_words) = word_occur else {
        return locations;
    };
    valid_words.sort_by_key(|w| w.1.len());
    let rarest = valid_words[0].clone();
    let index = words.iter().position(|&s| s == rarest.0).unwrap();
    let split_text: Vec<&str> = file.text.split_whitespace().collect(); // this is nasty, maybe we should consider the `File` object holding a split text

    for location in rarest.1 {
        let mut i: usize = 0;
        let location_usize = *location as usize;
        for word in &words {
            if location_usize >= index {
                if split_text[location_usize - index + i] != words[i] {
                    continue;
                }
            } else {
                return locations;
            }
            i += 1;
        }
        locations.push(*location);
    }
    locations
}
