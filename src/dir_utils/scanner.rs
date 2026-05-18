use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

use crate::supported_file::{MimeDetector, SupportedFile};

/// Configuration for the directory scanner.
pub struct ScannerConfig {
    /// Whether to follow symbolic links during scanning.
    pub follow_links: bool,
    /// Whether to include hidden files and directories in the scan.
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

/// Checks if a directory entry is hidden.
fn is_hidden(entry: &DirEntry) -> bool {
    let path = entry.path();
    hf::is_hidden(path).unwrap_or(false)
}

/// Recursively walks a directory and collects `SupportedFile`s.
///
/// Filters entries based on the provided `ScannerConfig` and uses the `detector`
/// to identify supported files.
fn from_dir<D: MimeDetector>(
    path: &Path,
    config: &ScannerConfig,
    detector: &D,
) -> Vec<SupportedFile> {
    WalkDir::new(path)
        .follow_links(config.follow_links)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) || config.include_hidden)
        .filter_map(|entry| match entry {
            Ok(e) => SupportedFile::from_path(e.into_path(), detector),
            Err(e) => {
                eprintln!("Warning: skipping entry: {e}");
                None
            }
        })
        .collect()
}

/// Collects `SupportedFile`s from a list of paths.
///
/// If a path is a directory, it scans it recursively based on the `config`.
/// If it's a file, it checks if it's a supported type using the `detector`.
pub fn get_fts_from_paths<D: MimeDetector>(
    paths: Vec<PathBuf>,
    config: &ScannerConfig,
    detector: &D,
) -> Vec<SupportedFile> {
    paths
        .into_iter()
        .flat_map(|path| {
            if path.is_dir() {
                from_dir(&path, config, detector)
            } else {
                SupportedFile::from_path(path, detector)
                    .into_iter()
                    .collect()
            }
        })
        .collect()
}
