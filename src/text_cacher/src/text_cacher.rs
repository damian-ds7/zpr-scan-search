mod text_cacher {
use std::fs::File;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::ffi::OsString;
    pub fn cache_text<P: AsRef<Path>>(text: &str, path: P) -> std::io::Result<File> {
        let map = create_word_map(text);
        Ok(save_text_and_map(text, &map, path)?)
    }

    fn create_word_map(text: &str) -> HashMap<String, Vec<i32>> {
        let mut cache_map: HashMap<String, Vec<i32>> = HashMap::new();
        let mut i = 0;
        for word in text.split_whitespace() {
            if let Some(v) = cache_map.get_mut(word) {
                v.push(i);
            } else {
                cache_map.insert(word.parse().unwrap(), vec![i]);
            }
            i += 1;
        }
        cache_map
    }


    fn save_text_and_map<P: AsRef<Path>>(text: &str, map: &HashMap<String, Vec<i32>>, path: P) -> std::io::Result<File> {
        let text_and_map = serde_json::to_string(map)? + text;
        let mut cache_path = path.as_ref().as_os_str().to_os_string();
        cache_path.push(".cache");
        let mut file = File::create(cache_path)?;
        file.write_all(text_and_map.as_bytes())?;
        Ok(file)
    }
}
#[cfg(test)]
mod tests {
    use std::io::Read;
    use super::*;
    use tempfile::tempdir;
    use crate::text_cacher::cache_text;

    #[test]
    fn test_cache_text() {
        let dir = tempdir().expect("Failed to create temp dir");
        let file_path = dir.path().join("my_document.pdf");
        let text = "Ala ma kota";
        let mut file = cache_text(text, &file_path).expect("Failed to cache text");
        let mut a = String::new();
        let content = file.read_to_string(&mut a);
        assert!(content.is_ok());
    }
}
