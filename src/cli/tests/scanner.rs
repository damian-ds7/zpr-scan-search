use std::{
    fs::{create_dir, write},
    path::Path,
};

use tempfile::TempDir;

use crate::{
    cli::scanner::{ScannerConfig, get_fts_from_paths},
    supported_file::MimeDetector,
};

pub struct MockDetector;

impl MimeDetector for MockDetector {
    fn detect(&self, path: &Path) -> Option<&'static str> {
        match path.extension()?.to_str()? {
            "pdf" => Some("application/pdf"),
            "png" => Some("image/png"),
            "jpg" => Some("image/jpeg"),
            "zip" => Some("application/zip"),
            _ => None,
        }
    }
}

/// Creates this structure:
/// root
/// ├── subdir
/// │   ├── nested
/// │   │   ├── archive.zip
/// │   │   └── report.pdf
/// │   ├── .hidden.pdf
/// │   ├── image.png
/// │   ├── noextension
/// │   └── photo.jpg
/// └── document.pdf
fn create_mock_dir() -> TempDir {
    let root = tempfile::tempdir().unwrap();

    write(root.path().join("document.pdf"), b"").unwrap();

    let subdir = root.path().join("subdir");
    create_dir(&subdir).unwrap();
    write(subdir.join("image.png"), b"").unwrap();
    write(subdir.join("photo.jpg"), b"").unwrap();
    write(subdir.join(".hidden.pdf"), b"").unwrap();
    write(subdir.join("noextension"), b"").unwrap();

    let nested = subdir.join("nested");
    create_dir(&nested).unwrap();
    write(nested.join("archive.zip"), b"").unwrap();
    write(nested.join("report.pdf"), b"").unwrap();

    root
}

#[test]
fn test_get_fts_from_paths_default_config() {
    let root = create_mock_dir();
    let config = ScannerConfig::default();

    let paths = vec![root.path().join("document.pdf"), root.path().join("subdir")];
    let result = get_fts_from_paths(paths, &config, &MockDetector);

    assert_eq!(result.len(), 4); // pdf, png, jpg, report.pdf
}

#[test]
fn test_get_fts_from_paths_include_hidden() {
    let root = create_mock_dir();
    let config = ScannerConfig {
        include_hidden: true,
        ..Default::default()
    };

    let paths = vec![root.path().join("document.pdf"), root.path().join("subdir")];
    let result = get_fts_from_paths(paths, &config, &MockDetector);

    dbg!(&result);

    assert_eq!(result.len(), 5); // pdf, png, jpg, .hidden.pdf, report.pdf
}
