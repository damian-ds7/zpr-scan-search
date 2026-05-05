use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::time::SystemTime;

use crate::error::{Result, ScanSearchError};

#[derive(Debug, PartialEq)]
pub struct FileFingerprint {
    mtime_secs: u64,
    mtime_nanos: u32,
    size: u64,
}

impl FileFingerprint {
    #[cfg(test)]
    pub fn new_raw(mtime_secs: u64, mtime_nanos: u32, size: u64) -> Self {
        Self {
            mtime_secs,
            mtime_nanos,
            size,
        }
    }
    pub fn from_path(path: &Path) -> Result<Self> {
        let meta = fs::metadata(path)?;
        let mtime = meta.modified()?;
        let duration = mtime
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| ScanSearchError::Io(std::io::Error::other(e)))?;
        Ok(Self {
            mtime_secs: duration.as_secs(),
            mtime_nanos: duration.subsec_nanos(),
            size: meta.len(),
        })
    }

    pub fn write_to(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(&self.mtime_secs.to_le_bytes())?;
        w.write_all(&self.mtime_nanos.to_le_bytes())?;
        w.write_all(&self.size.to_le_bytes())?;
        Ok(())
    }

    pub fn read_from(r: &mut impl Read) -> Result<Self> {
        let mut buf8 = [0u8; 8];
        let mut buf4 = [0u8; 4];
        r.read_exact(&mut buf8)?;
        let mtime_secs = u64::from_le_bytes(buf8);
        r.read_exact(&mut buf4)?;
        let mtime_nanos = u32::from_le_bytes(buf4);
        r.read_exact(&mut buf8)?;
        let size = u64::from_le_bytes(buf8);
        Ok(Self {
            mtime_secs,
            mtime_nanos,
            size,
        })
    }
}

impl Clone for FileFingerprint {
    fn clone(&self) -> Self {
        Self {
            mtime_secs: self.mtime_secs,
            mtime_nanos: self.mtime_nanos,
            size: self.size,
        }
    }
}
