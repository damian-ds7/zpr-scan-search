#[cfg(test)]
mod tests;
use crate::error::Result;
use crate::text_cacher::{cache_text, process_map, process_text};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct TextFile {
    pub path: PathBuf,
    pub text: String,
    pub map: std::collections::HashMap<String, Vec<i32>>,
}

impl TextFile {
    pub fn new(path: PathBuf) -> Result<TextFile> {
        if let Ok((text, map)) = Self::try_load_cache(&path) {
            return Ok(Self { path, text, map });
        }
        let mut file = TextFile {
            path,
            text: String::new(),
            map: HashMap::new(),
        };
        let text = Self::read_pdf(&file.path)?;
        cache_text(&text, &file.path.clone(), &mut file)?;
        Ok(file)
    }

    fn try_load_cache(path: &Path) -> Result<(String, HashMap<String, Vec<i32>>)> {
        let mut cache_path = path.to_path_buf();

        if let Some(file_name) = cache_path.file_name().and_then(|f| f.to_str()) {
            cache_path.set_file_name(format!("{}.cache", file_name));
        }

        let file = File::open(cache_path.as_path())?;
        let mut reader = BufReader::new(file);
        let map = process_map(&mut reader)?;
        let text = process_text(&mut reader)?;
        Ok((text, map))
    }

    fn read_pdf(path: &Path) -> Result<String> {
        unimplemented!()
    }
}
