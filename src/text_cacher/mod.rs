mod cache_writer;
mod file_fingerprint;
#[cfg(test)]
mod tests;

use crate::constants::DELIMITER;
use crate::error::Result;
use crate::text_cacher::cache_writer::Msg;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

pub use cache_writer::CacheWriter;
pub(crate) use cache_writer::Job;
pub use file_fingerprint::FileFingerprint;

pub struct CachedDocument {
    pub text: String,
    pub map: HashMap<String, Vec<i32>>,
    pub fingerprint: FileFingerprint,
}

/// Processes text into a map and triggers a background save to disk.
pub fn process_and_cache(
    text: String,
    path: PathBuf,
    fingerprint: FileFingerprint,
) -> (Arc<String>, Arc<HashMap<String, Vec<i32>>>) {
    let map = Arc::new(create_word_map(&text));
    let text = Arc::new(text);

    CacheWriter::get().submit(Msg::Write(
        (Job {
            text: Arc::clone(&text),
            map: Arc::clone(&map),
            fingerprint,
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

fn read_delimited<R: BufRead>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut buf = vec![];
    reader.read_until(DELIMITER, &mut buf)?;
    if buf.ends_with(&[DELIMITER]) {
        buf.pop();
    }
    Ok(buf)
}

/// Loads map, text and fingerprint parts from given cache file reader
pub fn load_parts<R: BufRead>(reader: &mut R) -> Result<CachedDocument> {
    let map = serde_json::from_slice(&read_delimited(reader)?)?;
    let text = String::from_utf8(read_delimited(reader)?)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let fingerprint = FileFingerprint::read_from(reader)?;

    Ok(CachedDocument {
        text,
        map,
        fingerprint,
    })
}
