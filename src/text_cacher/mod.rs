mod cache_writer;
pub(crate) mod codec;
mod file_fingerprint;
mod local_cache;
#[cfg(test)]
mod tests;
mod word_map;

use crate::error::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use cache_writer::CacheWriter;
pub use file_fingerprint::FileFingerprint;
pub use local_cache::LocalCache;
pub use word_map::WordMap;

pub trait CacheBackend {
    fn try_load(
        &self,
        path: &Path,
        fingerprint: &FileFingerprint,
    ) -> Result<Option<CachedDocument>>;

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

pub struct CachedDocument {
    pub text: String,
    pub map: WordMap,
    pub fingerprint: FileFingerprint,
}
