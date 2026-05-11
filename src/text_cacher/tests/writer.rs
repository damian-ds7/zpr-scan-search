use crate::error::ScanSearchError;
use crate::text_cacher::CacheWriter;
use crate::text_cacher::cache_writer::{self, Msg, WriteTask};
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_write_error_no_parent() {
    let task = WriteTask {
        path: PathBuf::from(""),
        data: b"some data".to_vec(),
    };

    let result = cache_writer::write(&task);
    assert!(matches!(result, Err(ScanSearchError::NoParentDir(_))));
}

#[test]
fn test_write_error_non_existent_dir() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("non_existent_folder/file.cache");

    let task = WriteTask {
        path: file_path,
        data: b"some data".to_vec(),
    };

    let result = cache_writer::write(&task);
    assert!(matches!(result, Err(ScanSearchError::Io(_))));
}

#[test]
fn test_writer_basic_write() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_basic.cache");
    let data = b"hello world".to_vec();

    let writer = CacheWriter::get();
    writer.submit(Msg::Write(WriteTask {
        path: file_path.clone(),
        data: data.clone(),
    }));

    writer.shutdown();

    assert!(file_path.exists());
    let read_data = fs::read(file_path).unwrap();
    assert_eq!(read_data, data);
}

#[test]
fn test_writer_multiple_writes() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("file1.cache");
    let file2 = dir.path().join("file2.cache");

    let writer = CacheWriter::get();

    writer.submit(Msg::Write(WriteTask {
        path: file1.clone(),
        data: b"data 1".to_vec(),
    }));
    writer.submit(Msg::Write(WriteTask {
        path: file2.clone(),
        data: b"data 2".to_vec(),
    }));

    writer.shutdown();

    assert_eq!(fs::read_to_string(file1).unwrap(), "data 1");
    assert_eq!(fs::read_to_string(file2).unwrap(), "data 2");
}

#[test]
fn test_writer_overwrite_existing() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("overwrite.cache");

    fs::write(&file_path, "initial data").unwrap();

    let writer = CacheWriter::get();
    writer.submit(Msg::Write(WriteTask {
        path: file_path.clone(),
        data: b"new data".to_vec(),
    }));

    writer.shutdown();

    assert_eq!(fs::read_to_string(file_path).unwrap(), "new data");
}
