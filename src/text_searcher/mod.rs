use std::collections::HashMap;
use rayon::iter::split;
use crate::file::TextFile;

#[cfg(test)]
pub mod tests;



pub fn search(file: &TextFile, query: &String) -> Result<i32, &'static str> {
    let words: Vec<&str> = query.split_whitespace().collect();
    let mut words_vec = vec![];
    for &word in &words {
        if let Some(value) = file.map.get(word) {
        words_vec.push((word, value.clone()));
        }
        else{
            return Err("Query not found");
        }
    }
    words_vec.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    let rarest = words_vec[0].clone();
    let index = words.iter().position(|&s| s == rarest.0).unwrap();
    let split_text: Vec<&str> = file.text.split_whitespace().collect();
    for location in rarest.1{
        let mut i: usize = 0;
        let location_usize = location as usize;
        for word in &words {
            if location_usize >= index{
                if split_text[location_usize - index + i] != words[i]{
                    continue;
                }
            }
            i += 1;
        }
        return Ok(location);
    }
    Err("Query not found")
}
