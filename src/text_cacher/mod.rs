mod cache_writer;
mod file_fingerprint;
mod local_cache;
#[cfg(test)]
mod tests;

use crate::constants::DELIMITER;
use crate::error::Result;
use std::collections::HashMap;
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use cache_writer::CacheWriter;
pub use file_fingerprint::FileFingerprint;
pub use local_cache::LocalCache;

pub type WordMap = HashMap<String, Vec<i32>>;

pub trait CacheBackend {
    fn try_load(
        &self,
        path: &Path,
        fingerprint: &FileFingerprint,
    ) -> Result<Option<CachedDocument>>;

    fn cache_document(
        &self,
        path: PathBuf,
        text: Arc<String>,
        map: Arc<WordMap>,
        fingerprint: FileFingerprint,
    );
}

/// Represents a single cache write task.
pub(crate) enum Job {
    CacheWrite {
        text: Arc<String>,
        map: Arc<WordMap>,
        fingerprint: FileFingerprint,
        path: PathBuf,
    },
}

pub fn execute_job<W: Write>(job: &Job, writer: &mut W) -> Result<()> {
    match job {
        Job::CacheWrite {
            text,
            map,
            fingerprint,
            path,
        } => serialize_cache_write(text, map, fingerprint, writer)?,
    }

    Ok(())
}

fn serialize_cache_write<W: Write>(
    text: &Arc<String>,
    map: &Arc<WordMap>,
    fingerprint: &FileFingerprint,
    writer: &mut W,
) -> Result<()> {
    serde_json::to_writer(&mut *writer, map.as_ref())?;
    writer.write_all(&[DELIMITER])?;
    writer.write_all(text.as_bytes())?;
    writer.write_all(&[DELIMITER])?;
    write_fingerprint(fingerprint, writer)?;
    Ok(())
}

pub(crate) fn write_fingerprint<W: Write>(fingerprint: &FileFingerprint, w: &mut W) -> Result<()> {
    w.write_all(&fingerprint.mtime_secs.to_le_bytes())?;
    w.write_all(&fingerprint.mtime_nanos.to_le_bytes())?;
    w.write_all(&fingerprint.size.to_le_bytes())?;
    Ok(())
}

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

    backend.cache_document(path, Arc::clone(&text), Arc::clone(&map), fingerprint);

    (text, map)
}

/// Creates a map of words and their occurrence indices in the text.
pub fn create_word_map(text: &str) -> WordMap {
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
