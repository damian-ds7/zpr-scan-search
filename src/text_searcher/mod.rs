use crate::file::TextFile;
use rayon::iter::split;
use std::collections::HashMap;

#[cfg(test)]
pub mod tests;

pub fn search(file: &TextFile, query: &String) -> Vec<i32> {
    let words: Vec<&str> = query.split_whitespace().collect();
    let mut words_vec = vec![];
    let mut locations: Vec<i32> = vec![];
    for &word in &words {
        if let Some(value) = file.map.get(word) {
            words_vec.push((word, value.clone()));
        } else {
            return locations;
        }
    }
    words_vec.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    let rarest = words_vec[0].clone();
    let index = words.iter().position(|&s| s == rarest.0).unwrap();
    let split_text: Vec<&str> = file.text.split_whitespace().collect(); // this is nasty, maybe we should consider the `File` object holding a split text

    for location in rarest.1 {
        let mut i: usize = 0;
        let location_usize = location as usize;
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
        locations.push(location);
    }
    locations
}
