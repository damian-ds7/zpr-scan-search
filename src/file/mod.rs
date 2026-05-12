mod loader;
#[cfg(test)]
mod tests;
use crate::text_cacher::WordMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use loader::TextFileLoader;
use crate::text_encoder::TextEncoder;

/// Represents a processed document containing its text content and a word occurrence map.
#[allow(dead_code)] // TODO: check if path can be removed later
pub struct TextFile {
    path: PathBuf,
    text: Arc<String>,
    map: Arc<WordMap>,
    pub(crate) embeddings: Option<Arc<Vec<Vec<f32>>>>
}

impl TextFile {
    pub fn new(path: PathBuf, text: String, map: WordMap) -> Self {
        Self {
            path,
            text: Arc::new(text),
            map: Arc::new(map),
            embeddings: None,
        }
    }

    #[allow(dead_code)] // TODO: check if path can be removed later
    fn path(&self) -> &Path {
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

    pub fn embeddings(&self) -> &Option<Arc<Vec<Vec<f32>>>> {&self.embeddings}

    pub fn set_embeddings<E: TextEncoder>(&mut self, encoder: &E) {
        self.embeddings = encoder
            .encode(&self.text.lines().collect())
            .ok()
            .map(Arc::new);
    }
}
