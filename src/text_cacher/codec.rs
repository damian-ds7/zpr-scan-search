use crate::constants::DELIMITER;
use crate::error::Result;
use crate::text_cacher::{CachedDocument, Embeddings, FileFingerprint, WordMap, read_embeddings};
use std::io::{self, BufRead, Write};
use std::sync::Arc;

/// Serializes the word map, text, and file fingerprint to the provided writer.
pub(crate) fn serialize_cache_write<W: Write>(
    text: &Arc<String>,
    map: &Arc<WordMap>,
    fingerprint: &FileFingerprint,
    writer: &mut W,
    embeddings: &Arc<Option<Embeddings>>,
) -> Result<()> {
    serde_json::to_writer(&mut *writer, map.as_ref())?;
    writer.write_all(&[DELIMITER])?;
    writer.write_all(text.as_bytes())?;
    writer.write_all(&[DELIMITER])?;
    match embeddings.as_ref() {
        Some(emb) => {
            serde_json::to_writer(&mut *writer, emb)?;
        }
        None => {}
    }
    writer.write_all(&[DELIMITER])?;
    write_fingerprint(fingerprint, writer)?;
    Ok(())
}

/// Writes the file fingerprint (mtime and size) to the provided writer.
pub(crate) fn write_fingerprint<W: Write>(fingerprint: &FileFingerprint, w: &mut W) -> Result<()> {
    w.write_all(&fingerprint.mtime_secs.to_le_bytes())?;
    w.write_all(&fingerprint.mtime_nanos.to_le_bytes())?;
    w.write_all(&fingerprint.size.to_le_bytes())?;
    Ok(())
}

/// Processes text into a map and triggers a background save to disk.
pub fn process_text(text: String) -> (Arc<String>, Arc<WordMap>) {
    let map = Arc::new(WordMap::from(&text));
    let text = Arc::new(text);
    (text, map)
}

/// Reads from the reader until the delimiter is encountered.
pub(crate) fn read_delimited<R: BufRead>(reader: &mut R) -> io::Result<Vec<u8>> {
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
    let embeddings = read_embeddings(crate::text_cacher::read_delimited(reader)?)?;
    let fingerprint = read_fingerprint(reader)?;

    Ok(CachedDocument {
        text,
        map,
        fingerprint,
        embeddings: Some(embeddings),
    })
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
