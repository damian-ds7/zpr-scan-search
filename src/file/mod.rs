mod loader;
#[cfg(test)]
mod tests;
use crate::text_cacher::{Embeddings, WordMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use loader::{FileLoader, TextFileLoader};

/// Represents a processed document containing its text content and a word occurrence map.
#[allow(dead_code)] // TODO: check if path can be removed later
pub struct TextFile {
    path: PathBuf,
    text: Arc<str>,
    map: Arc<WordMap>,
    pub(crate) embeddings: Arc<Option<Embeddings>>,
}

impl TextFile {
    pub fn new(path: PathBuf, text: String, map: WordMap, embeddings: Option<Embeddings>) -> Self {
        Self {
            path,
            text: text.into_boxed_str().into(),
            map: Arc::new(map),
            embeddings: Arc::new(embeddings),
        }
    }

    #[allow(dead_code)] // TODO: check if path can be removed later
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn get(&self, key: &str) -> Option<&Vec<i32>> {
        self.map.get(key)
    }

    // TODO: can probably be removed after preview python function is not needed
    pub fn map(&self) -> &WordMap {
        &self.map
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_arc(&self) -> Arc<str> {
        Arc::clone(&self.text)
    }

    pub fn embeddings(&self) -> &Arc<Option<Embeddings>> {
        &self.embeddings
    }
}
