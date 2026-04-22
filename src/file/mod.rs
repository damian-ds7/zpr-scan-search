use std::path::PathBuf;
use std::fs;
use filetime::FileTime;
use std::io;

pub mod file {
    use super::*;
    pub struct File<'a> {
        pub path: PathBuf,
        text: &'a str,
        map: std::collections::HashMap<String, Vec<i32>>,
    }

    impl<'a> File<'a> {
        pub fn new(path: PathBuf) -> File<'a> {

        }
        fn check_cache(&self) -> io::Result<bool>{
            let read_content = fs::read_to_string(self.path)?;
            let inverted_idx = process_map(read_content)
        }

    }
}
