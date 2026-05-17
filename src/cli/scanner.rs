use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

use crate::filetype::FileType;

pub struct ScannerConfig {
    pub follow_links: bool,
    pub include_hidden: bool,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            follow_links: true,
            include_hidden: false,
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    let path = entry.path();
    hf::is_hidden(path).unwrap_or(false)
}

/// Walks a directory and collects FileTypes
fn from_dir(path: &Path, config: &ScannerConfig) -> Vec<FileType> {
    WalkDir::new(path)
        .follow_links(config.follow_links)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) || config.include_hidden)
        .filter_map(|entry| match entry {
            Ok(e) => FileType::from_path(e.into_path()),
            Err(e) => {
                eprintln!("Warning: skipping entry: {e}");
                None
            }
        })
        .collect()
}

pub fn get_fts_from_paths(paths: Vec<PathBuf>, config: &ScannerConfig) -> Vec<FileType> {
    paths
        .into_iter()
        .flat_map(|path| {
            if path.is_dir() {
                from_dir(&path, config)
            } else {
                FileType::from_path(path).into_iter().collect()
            }
        })
        .collect()
}
