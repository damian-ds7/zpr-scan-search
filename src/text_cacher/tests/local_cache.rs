use std::fs::File;
use std::sync::Arc;
use tempfile::tempdir;

use crate::text_cacher::{
    CacheBackend, FileFingerprint, LocalCache, WordMap, serialize_cache_write,
};

#[test]
fn test_local_cache_valid_cache() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("document.pdf");
    let cache_path = dir.path().join("document.pdf.cache");

    let fp = FileFingerprint::new_raw(1234, 5678, 999);
    let text = Arc::new("cached content".to_string());
    let mut map = WordMap::new();
    map.insert("cached".to_string(), vec![0]);
    let map_arc = Arc::new(map);

    // Manually create a valid cache file
    let mut file = File::create(&cache_path).unwrap();
    serialize_cache_write(&text, &map_arc, &fp, &mut file).unwrap();

    let backend = LocalCache::new();
    let result = backend.try_load(&file_path, &fp).unwrap();

    assert!(result.is_some());
    let doc = result.unwrap();
    assert_eq!(doc.text, "cached content");
    assert_eq!(doc.map.get("cached").unwrap(), &vec![0]);
    assert_eq!(doc.fingerprint, fp);
}

#[test]
fn test_local_cache_no_cache() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("missing.pdf");
    let fp = FileFingerprint::new_raw(1, 2, 3);

    let backend = LocalCache::new();
    let result = backend.try_load(&file_path, &fp).unwrap();

    assert!(result.is_none());
}

#[test]
fn test_local_cache_fingerprint_mismatch() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("stale.pdf");
    let cache_path = dir.path().join("stale.pdf.cache");

    let fp_old = FileFingerprint::new_raw(1, 1, 1);
    let fp_new = FileFingerprint::new_raw(2, 2, 2);

    let text = Arc::new("old content".to_string());
    let map = Arc::new(WordMap::new());

    // Create cache with old fingerprint
    let mut file = File::create(&cache_path).unwrap();
    serialize_cache_write(&text, &map, &fp_old, &mut file).unwrap();

    let backend = LocalCache::new();
    // Try to load with new fingerprint
    let result = backend.try_load(&file_path, &fp_new).unwrap();

    assert!(
        result.is_none(),
        "Should return None if fingerprint does not match"
    );
}
