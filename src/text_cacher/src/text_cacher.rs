mod text_cacher {
use std::fs::File;
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};

const DELIMITER: &str = "-----EOF-----";
    pub fn cache_text<P: AsRef<Path>>(text: &str, path: P) -> std::io::Result<PathBuf> {
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


    fn save_text_and_map<P: AsRef<Path>>(text: &str, map: &HashMap<String, Vec<i32>>, path: P) -> std::io::Result<PathBuf> {
        let text_and_map = serde_json::to_string(map)? + DELIMITER + text;
        let mut cache_path_string = path.as_ref().as_os_str().to_os_string();
        cache_path_string.push(".cache");
        let final_path = PathBuf::from(cache_path_string);
        let mut file = File::create(&final_path)?;
        file.write_all(text_and_map.as_bytes())?;
        Ok(final_path)
    }
}
#[cfg(test)]
mod tests {
    use std::io::Read;
    use tempfile::tempdir;
    use crate::text_cacher::cache_text;
    use std::fs::File;
    use std::collections::HashMap;
    const DELIMITER: &str = "-----EOF-----";
    #[test]
    fn test_cache_text_multiple_cases() {
        let test_cases = [
            ("Ala ma kota", "{\"Ala\":[0],\"kota\":[2],\"ma\":[1]}"),
            ("Ala ma kota kota", "{\"Ala\":[0],\"kota\":[2,3],\"ma\":[1]}"),
            ("hello hello hello", "{\"hello\":[0,1,2]}"),
            ("", "{}")
        ];

        for (text, expected_json_str) in test_cases {
            let dir = tempdir().expect("Failed to create temp dir");
            let file_path = dir.path().join("my_document.pdf");
            let cache_path = cache_text(text, &file_path).expect("Failed to cache text");
            let mut file = File::open(cache_path).expect("Failed to open cache file");
            let mut a = String::new();
            let content = file.read_to_string(&mut a);
            assert!(content.is_ok());
            let (json_str, text_str) = a.split_once(DELIMITER).expect("Delimiter not found");
            let parsed_map: HashMap<String, Vec<i32>> = serde_json::from_str(json_str).expect("Failed to parse map");
            let expected_map: HashMap<String, Vec<i32>> = serde_json::from_str(expected_json_str).expect("Failed to parse expected map");
            assert_eq!(parsed_map, expected_map, "Failed on input: {}", text);
            assert_eq!(text_str, text, "Text mismatch on input: {}", text);
        }
    }
}
