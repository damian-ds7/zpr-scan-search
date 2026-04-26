#[cfg(test)]
mod tests;
use crate::error::Result;
use crate::text_cacher::{load_parts, process_and_cache};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct TextFile {
    pub path: PathBuf,
    pub text: Arc<String>,
    pub map: Arc<HashMap<String, Vec<i32>>>,
}

impl TextFile {
    pub fn new(path: PathBuf) -> Result<TextFile> {
        if let Ok((text, map)) = Self::try_load_cache(&path) {
            return Ok(Self {
                path,
                text: Arc::new(text),
                map: Arc::new(map),
            });
        }

        let text = Self::read_pdf(&path)?;
        let (text, map) = process_and_cache(text, path.clone());
        Ok(Self { path, text, map })
    }

    fn try_load_cache(path: &Path) -> Result<(String, HashMap<String, Vec<i32>>)> {
        let mut cache_path = path.to_path_buf();

        if let Some(file_name) = cache_path.file_name().and_then(|f| f.to_str()) {
            cache_path.set_file_name(format!("{}.cache", file_name));
        }

        let file = File::open(cache_path.as_path())?;
        let mut reader = BufReader::new(file);
        load_parts(&mut reader)
    }

    fn read_pdf(path: &Path) -> Result<String> {
        unimplemented!()
    }
}
