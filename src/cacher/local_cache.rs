use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::cacher::cache_writer::{Msg, WriteTask};
use crate::cacher::{
    CacheBackend, CacheWriter, CachedDocument, Embeddings, FileFingerprint, Job, WordMap,
};
use crate::constants::DELIMITER;
use crate::error::Result;

/// A local file-system based cache backend.
/// It stores cache files in the same directory as the original file, appending `.cache` to the filename.
#[derive(Default)]
pub struct LocalCache;

impl LocalCache {
    /// Serializes the word map, text, and file fingerprint to the provided writer.
    pub(crate) fn serialize<W: Write>(
        text: &Arc<str>,
        map: &Arc<WordMap>,
        fingerprint: &FileFingerprint,
        writer: &mut W,
        embeddings: &Arc<Option<Embeddings>>,
    ) -> Result<()> {
        serde_json::to_writer(&mut *writer, map.as_ref())?;
        writer.write_all(&[DELIMITER])?;
        writer.write_all(text.as_bytes())?;
        writer.write_all(&[DELIMITER])?;
        serde_json::to_writer(&mut *writer, embeddings.as_ref())?;
        writer.write_all(&[DELIMITER])?;
        Self::write_fingerprint(fingerprint, writer)?;
        Ok(())
    }

    /// Loads map, text and fingerprint parts from given cache file reader
    pub(crate) fn deserialize<R: BufRead>(reader: &mut R) -> Result<CachedDocument> {
        let map = serde_json::from_slice(&Self::read_delimited(reader)?)?;
        let text = String::from_utf8(Self::read_delimited(reader)?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let embeddings = serde_json::from_slice(&Self::read_delimited(reader)?)?;
        let fingerprint = Self::read_fingerprint(reader)?;

        Ok(CachedDocument {
            text,
            map,
            fingerprint,
            embeddings,
        })
    }

    /// Writes the file fingerprint (mtime and size) to the provided writer.
    pub(crate) fn write_fingerprint<W: Write>(
        fingerprint: &FileFingerprint,
        w: &mut W,
    ) -> Result<()> {
        w.write_all(&fingerprint.mtime_secs.to_le_bytes())?;
        w.write_all(&fingerprint.mtime_nanos.to_le_bytes())?;
        w.write_all(&fingerprint.size.to_le_bytes())?;
        Ok(())
    }

    /// Reads the file fingerprint from the provided reader.
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

    /// Reads from the reader until the delimiter is encountered.
    fn read_delimited<R: BufRead>(reader: &mut R) -> io::Result<Vec<u8>> {
        let mut buf = vec![];
        reader.read_until(DELIMITER, &mut buf)?;
        if buf.ends_with(&[DELIMITER]) {
            buf.pop();
        }
        Ok(buf)
    }
}

impl CacheBackend for LocalCache {
    fn try_load(
        &self,
        path: &Path,
        fingerprint: &FileFingerprint,
        reload_cache: bool,
    ) -> Result<Option<CachedDocument>> {
        if reload_cache {
            return Ok(None);
        }

        let mut cache_path = path.to_path_buf();

        if let Some(file_name) = cache_path.file_name().and_then(|f| f.to_str()) {
            cache_path.set_file_name(format!("{}.cache", file_name));
        }

        let file = match File::open(cache_path.as_path()) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let mut reader = BufReader::new(file);
        let cached_file = Self::deserialize(&mut reader)?;

        if cached_file.fingerprint != *fingerprint {
            return Ok(None);
        }

        Ok(Some(cached_file))
    }

    fn submit_job(&self, path: PathBuf, job: Job) {
        let mut cache_path = path;
        if let Some(file_name) = cache_path.file_name().and_then(|f| f.to_str()) {
            cache_path.set_file_name(format!("{}.cache", file_name));
        }

        match job {
            Job::CacheWrite {
                text,
                map,
                fingerprint,
                embeddings,
            } => {
                let mut data = Vec::new();
                if let Err(e) = Self::serialize(&text, &map, &fingerprint, &mut data, &embeddings) {
                    eprintln!("Failed to serialize cache data for {:?}: {}", cache_path, e);
                    return;
                }

                CacheWriter::get().submit(Msg::Write(WriteTask {
                    data,
                    path: cache_path,
                }));
            }
        }
    }
}
