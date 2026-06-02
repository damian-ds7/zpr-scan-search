mod loader;
#[cfg(test)]
mod tests;
use crate::cacher::{Embeddings, WordMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use loader::{FileLoader, TextFileLoader};

/// Represents a processed document containing its text content, word map and optionally embeddings
///
/// It is purely a data class, loading functionality is done by the Loader
#[allow(dead_code)] // TODO: check if path can be removed later
pub struct TextFile {
    path: PathBuf,
    text: Arc<str>,
    map: Arc<WordMap>,
    pub(crate) embeddings: Arc<Option<Embeddings>>,
}

impl TextFile {
    pub fn new(
        path: PathBuf,
        text: Arc<str>,
        map: Arc<WordMap>,
        embeddings: Arc<Option<Embeddings>>,
    ) -> Self {
        Self {
            path,
            text,
            map,
            embeddings,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn get(&self, key: &str) -> Option<&Vec<i32>> {
        self.map.get(key)
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_arc(&self) -> Arc<str> {
        Arc::clone(&self.text)
    }
}
