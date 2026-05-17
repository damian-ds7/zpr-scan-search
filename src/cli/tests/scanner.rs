use std::fs::{create_dir, write};

use tempfile::TempDir;

use crate::cli::{get_fts_from_paths, scanner::ScannerConfig};

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
    let result = get_fts_from_paths(paths, &config);

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
    let result = get_fts_from_paths(paths, &config);

    dbg!(&result);

    assert_eq!(result.len(), 5); // pdf, png, jpg, .hidden.pdf, report.pdf
}
