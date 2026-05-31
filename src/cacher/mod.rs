mod cache_writer;
mod embeddings;
mod file_fingerprint;
mod local_cache;
#[cfg(test)]
mod tests;
mod word_map;

use crate::error::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use cache_writer::CacheWriter;
pub use embeddings::Embeddings;
pub use file_fingerprint::FileFingerprint;
pub use global_cache::GlobalCache;
pub use local_cache::LocalCache;
pub use word_map::WordMap;

/// Processes text into a map and triggers a background save to disk.
pub(crate) fn process_text(text: String) -> (Arc<str>, Arc<WordMap>) {
    let map = Arc::new(WordMap::from(&text));
    let text = text.into_boxed_str().into();
    (text, map)
}

/// Interface for cache backends that store and retrieve processed document data.
pub trait CacheBackend: Sync + Send {
    /// Attempts to load a cached document for the given path and fingerprint.
    fn try_load(
        &self,
        path: &Path,
        fingerprint: &FileFingerprint,
        reload_cache: bool,
    ) -> Result<Option<CachedDocument>>;

    /// Submits a background job to the cache backend.
    fn submit_job(&self, path: PathBuf, job: Job);
}

/// Represents the high-level domain data for a cache write task.
pub enum Job {
    CacheWrite {
        text: Arc<str>,
        map: Arc<WordMap>,
        fingerprint: FileFingerprint,
        embeddings: Arc<Option<Embeddings>>,
    },
}

/// Represents a document loaded from the cache.
#[derive(Clone, Debug)]
pub struct CachedDocument {
    pub text: String,
    pub map: WordMap,
    pub fingerprint: FileFingerprint,
    pub embeddings: Option<Embeddings>,
}
