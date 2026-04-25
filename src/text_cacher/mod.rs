#[cfg(test)]
mod tests;

use crate::constants::DELIMITER;
use crate::file::TextFile;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn cache_text<P: AsRef<Path>>(
    text: &str,
    path: &P,
    file: &mut TextFile,
) -> std::io::Result<PathBuf> {
    let map = create_word_map(text);
    file.map = map;
    file.text = text.to_string();
    save_text_and_map(text, &file.map, path)
}

pub fn process_map(reader: &mut BufReader<File>) -> io::Result<HashMap<String, Vec<i32>>> {
    let mut buf = vec![];
    reader.read_until(DELIMITER, &mut buf)?;
    if buf.ends_with(&[DELIMITER]) {
        buf.pop();
    }
    serde_json::from_slice::<HashMap<String, Vec<i32>>>(&buf)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn process_text(reader: &mut BufReader<File>) -> io::Result<String> {
    let mut buf = vec![];
    reader.read_until(DELIMITER, &mut buf)?;
    if buf.ends_with(&[DELIMITER]) {
        buf.pop();
    }
    let text = String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(text)
}

fn create_word_map(text: &str) -> HashMap<String, Vec<i32>> {
    let mut cache_map: HashMap<String, Vec<i32>> = HashMap::new();
    let mut i = 0;
    text.split_whitespace().for_each(|word| {
        if let Some(v) = cache_map.get_mut(word) {
            v.push(i);
        } else {
            cache_map.insert(word.parse().unwrap(), vec![i]);
        }
        i += 1;
    });
    cache_map
}

fn save_text_and_map<P: AsRef<Path>>(
    text: &str,
    map: &HashMap<String, Vec<i32>>,
    path: P,
) -> io::Result<PathBuf> {
    let mut map_string = serde_json::to_string(map)?;
    map_string.push(DELIMITER as char);
    let text_and_map = map_string + text;
    let mut cache_path_string = path.as_ref().as_os_str().to_os_string();
    cache_path_string.push(".cache");
    let final_path = PathBuf::from(cache_path_string);
    let mut file = File::create(&final_path)?;
    file.write_all(text_and_map.as_bytes())?;
    Ok(final_path)
}
