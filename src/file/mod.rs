use super::*;
use crate::text_cacher::{cache_text, process_map, process_text};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;

pub struct TextFile {
    pub path: PathBuf,
    pub text: String,
    pub map: std::collections::HashMap<String, Vec<i32>>,
}

impl TextFile {
    pub fn new(path: PathBuf) -> io::Result<TextFile> {
        let mut file = TextFile {
            path,
            text: String::new(),
            map: HashMap::new(),
        };
        let text = file.read_pdf();
        if let Err(_) = file.check_cache() {
            cache_text(&text, &file.path.clone(), &mut file)?;
        }
        Ok(file)
    }

    fn check_cache(&mut self) -> io::Result<()> {
        let mut cache_path = self.path.clone();
        cache_path.push(".cache");
        let file = fs::File::open(cache_path.as_path())?;
        let mut reader = BufReader::new(file);
        let inverted_index_map = process_map(&mut reader)?;
        let text = process_text(&mut reader)?;
        self.map = inverted_index_map;
        self.text = text;
        Ok(())
    }

    fn read_pdf(&mut self) -> String {
        unimplemented!()
    }
}
