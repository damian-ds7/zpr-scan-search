mod cache_writer;
pub(crate) mod codec;
mod file_fingerprint;
mod local_cache;
#[cfg(test)]
mod tests;
mod word_map;

mod embeddings;

use crate::constants::DELIMITER;
use crate::error::Result;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use cache_writer::CacheWriter;
pub use file_fingerprint::FileFingerprint;
pub use local_cache::LocalCache;
pub use word_map::WordMap;
pub use embeddings::Embeddings;

pub trait CacheBackend {
    /// Attempts to load a cached document for the given path and fingerprint.
    fn try_load(
        &self,
        path: &Path,
        fingerprint: &FileFingerprint,
    ) -> Result<Option<CachedDocument>>;

    /// Submits a background job to the cache backend.
    fn submit_job(&self, path: PathBuf, job: Job);
}

/// Represents the high-level domain data for a cache write task.
pub enum Job {
    CacheWrite {
        text: Arc<String>,
        map: Arc<WordMap>,
        fingerprint: FileFingerprint,
    },
}

/// Represents a document loaded from the cache.
pub struct CachedDocument {
    pub text: String,
    pub map: WordMap,
    pub fingerprint: FileFingerprint,
}

/// Processes text into a map and triggers a background save to disk.
pub fn process_and_cache<B: CacheBackend>(
    text: String,
    path: PathBuf,
    fingerprint: FileFingerprint,
    backend: &B,
) -> (Arc<String>, Arc<WordMap>) {
    let map = Arc::new(create_word_map(&text));
    let text = Arc::new(text);

    backend.submit_job(
        path,
        Job::CacheWrite {
            text: Arc::clone(&text),
            map: Arc::clone(&map),
            fingerprint,
        },
    );

    (text, map)
}

/// Creates a map of words and their occurrence indices in the text.
pub fn create_word_map(text: &str) -> WordMap {
    text.split_whitespace()
        .enumerate()
        .fold(WordMap::new(), |mut acc, (i, word)| {
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
    let fingerprint = read_fingerprint(reader)?;

    Ok(CachedDocument {
        text,
        map,
        fingerprint,
    })
}

pub(crate) fn read_fingerprint<R: BufRead>(r: &mut R) -> Result<FileFingerprint> {
    let mut buf8 = [0u8; 8];
    let mut buf4 = [0u8; 4];
    r.read_exact(&mut buf8)?;
    let mtime_secs = u64::from_le_bytes(buf8);
    r.read_exact(&mut buf4)?;
    let mtime_nanos = u32::from_le_bytes(buf4);
    r.read_exact(&mut buf8)?;
    let size = u64::from_le_bytes(buf8);
    Ok(FileFingerprint {
        mtime_secs,
        mtime_nanos,
        size,
    })
}
