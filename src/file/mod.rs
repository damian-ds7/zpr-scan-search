mod loader;
#[cfg(test)]
mod tests;
use crate::error::Result;
use crate::text_cacher::process_and_cache;
use crate::text_cacher::{CacheBackend, CachedDocument, FileFingerprint, LocalCache, WordMap};
use crate::text_extractor::TextExtractor;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use loader::TextFileLoader;

/// Represents a processed document containing its text content and a word occurrence map.
pub struct TextFile {
    path: PathBuf,
    text: Arc<String>,
    map: Arc<WordMap>,
}

impl TextFile {
    pub fn new(path: PathBuf, text: String, map: WordMap) -> Self {
        Self {
            path,
            text: Arc::new(text),
            map: Arc::new(map),
        }
    }

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
}
