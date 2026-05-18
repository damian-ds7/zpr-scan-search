mod cache_writer;
pub(crate) mod codec;
mod file_fingerprint;
mod local_cache;
#[cfg(test)]
mod tests;
mod word_map;

mod embeddings;

use crate::error::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use cache_writer::CacheWriter;
pub use embeddings::Embeddings;
pub use file_fingerprint::FileFingerprint;
pub use local_cache::LocalCache;
pub use word_map::WordMap;

/// Interface for cache backends that store and retrieve processed document data.
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
        embeddings: Arc<Option<Embeddings>>
    },
}

/// Represents a document loaded from the cache.
pub struct CachedDocument {
    pub text: String,
    pub map: WordMap,
    pub fingerprint: FileFingerprint,
    pub embeddings: Option<Embeddings>
}
