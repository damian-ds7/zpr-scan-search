use std::fs;
use std::path::Path;
use std::time::SystemTime;

use crate::error::{Result, ScanSearchError};

#[derive(Debug, PartialEq, Clone)]
pub struct FileFingerprint {
    pub(crate) mtime_secs: u64,
    pub(crate) mtime_nanos: u32,
    pub(crate) size: u64,
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
}
