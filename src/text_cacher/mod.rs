mod cache_writer;
#[cfg(test)]
mod tests;

use crate::constants::DELIMITER;
use crate::error::Result;
use crate::text_cacher::cache_writer::{Job, Msg};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

pub use cache_writer::CacheWriter;

/// Processes text into a map and triggers a background save to disk.
pub fn process_and_cache(
    text: String,
    path: PathBuf,
) -> (Arc<String>, Arc<HashMap<String, Vec<i32>>>) {
    let map = Arc::new(create_word_map(&text));
    let text = Arc::new(text);

    CacheWriter::get().submit(Msg::Write(
        (Job {
            text: Arc::clone(&text),
            map: Arc::clone(&map),
            path,
        }),
    ));

    (text, map)
}

/// Creates a map of words and their occurrence indices in the text.
pub fn create_word_map(text: &str) -> HashMap<String, Vec<i32>> {
    text.split_whitespace()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, word)| {
            acc.entry(word.to_string()).or_default().push(i as i32);
            acc
        })
}

/// Loads map and text parts from given cache file reader
pub fn load_parts(reader: &mut BufReader<File>) -> Result<(String, HashMap<String, Vec<i32>>)> {
    let mut buf = vec![];

    reader.read_until(DELIMITER, &mut buf)?;
    if buf.ends_with(&[DELIMITER]) {
        buf.pop();
    }
    let map = serde_json::from_slice(&buf)?;

    buf.clear();
    reader.read_to_end(&mut buf)?;
    let text = String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok((text, map))
}
