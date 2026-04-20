#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const DELIMITER: &str = "-----EOF-----";

pub fn cache_text<P: AsRef<Path>>(text: &str, path: P) -> std::io::Result<PathBuf> {
    let map = create_word_map(text);
    Ok(save_text_and_map(text, &map, path)?)
}

fn create_word_map(text: &str) -> HashMap<String, Vec<i32>> {
    let mut cache_map: HashMap<String, Vec<i32>> = HashMap::new();
    let mut i = 0;
    for word in text.split_whitespace() {
        if let Some(v) = cache_map.get_mut(word) {
            v.push(i);
        } else {
            cache_map.insert(word.parse().unwrap(), vec![i]);
        }
        i += 1;
    }
    cache_map
}

fn save_text_and_map<P: AsRef<Path>>(
    text: &str,
    map: &HashMap<String, Vec<i32>>,
    path: P,
) -> std::io::Result<PathBuf> {
    let text_and_map = serde_json::to_string(map)? + DELIMITER + text;
    let mut cache_path_string = path.as_ref().as_os_str().to_os_string();
    cache_path_string.push(".cache");
    let final_path = PathBuf::from(cache_path_string);
    let mut file = File::create(&final_path)?;
    file.write_all(text_and_map.as_bytes())?;
    Ok(final_path)
}
