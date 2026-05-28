use std::fs::File;
use std::sync::Arc;
use tempfile::tempdir;

use crate::text_cacher::{
    CacheBackend, Embeddings, FileFingerprint, LocalCache, WordMap,
    codec::{process_text, serialize_cache_write},
};

#[test]
fn test_local_cache_valid_cache() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("document.pdf");
    let cache_path = dir.path().join("document.pdf.cache");

    let fp = FileFingerprint {
        mtime_secs: 1234,
        mtime_nanos: 5678,
        size: 999,
    };

    let text = Arc::from("cached content".to_string().into_boxed_str());
    let mut map = WordMap::new();
    map.insert("cached".to_string(), vec![0]);
    let map_arc = Arc::new(map);
    // Manually create a valid cache file
    let mut file = File::create(&cache_path).unwrap();
    let embeddings = Arc::new(Some(Embeddings::from(vec![
        vec![0.1, 0.2, 0.3],
        vec![0.4, 0.5, 0.6],
    ])));
    serialize_cache_write(&text, &map_arc, &fp, &mut file, &embeddings).unwrap();

    let backend = LocalCache;
    let result = backend.try_load(&file_path, &fp).unwrap();

    assert!(result.is_some());
    let doc = result.unwrap();
    assert_eq!(doc.text, "cached content");
    assert_eq!(doc.map.get("cached").unwrap(), &vec![0]);
    assert_eq!(doc.fingerprint, fp);
    let emb = doc.embeddings.expect("embeddings should be Some");
    assert_eq!(emb.len(), 2);
    assert_eq!(emb[0], vec![0.1, 0.2, 0.3]);
    assert_eq!(emb[1], vec![0.4, 0.5, 0.6]);
}

#[test]
fn test_local_cache_no_cache() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("missing.pdf");

    let fp = FileFingerprint {
        mtime_secs: 1234,
        mtime_nanos: 5678,
        size: 999,
    };

    let backend = LocalCache;
    let result = backend.try_load(&file_path, &fp).unwrap();

    assert!(result.is_none());
}

#[test]
fn test_local_cache_fingerprint_mismatch() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("stale.pdf");
    let cache_path = dir.path().join("stale.pdf.cache");

    let fp_old = FileFingerprint {
        mtime_secs: 1234,
        mtime_nanos: 5678,
        size: 999,
    };
    let fp_new = FileFingerprint {
        mtime_secs: 1,
        mtime_nanos: 1,
        size: 1,
    };

    let text = Arc::from("old content".to_string().into_boxed_str());
    let map = Arc::new(WordMap::new());
    let embeddings = Arc::new(Some(Embeddings::from(vec![vec![1.0, 2.0]])));
    // Create cache with old fingerprint
    let mut file = File::create(&cache_path).unwrap();
    serialize_cache_write(&text, &map, &fp_old, &mut file, &embeddings).unwrap();

    let backend = LocalCache;
    // Try to load with new fingerprint
    let result = backend.try_load(&file_path, &fp_new).unwrap();

    assert!(
        result.is_none(),
        "Should return None if fingerprint does not match"
    );
}

#[test]
fn test_local_cache_round_trip() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("roundtrip.pdf");
    let fp = FileFingerprint {
        mtime_secs: 1234,
        mtime_nanos: 5678,
        size: 999,
    };

    let text = "round trip content".to_string();
    let (text_arc, map_arc) = process_text(text);
    let embeddings = Arc::new(Some(Embeddings::from(vec![
        vec![0.7, 0.8, 0.9],
        vec![1.0, 1.1, 1.2],
        vec![1.3, 1.4, 1.5],
    ])));
    let backend = LocalCache;

    backend.submit_job(
        file_path.clone(),
        crate::text_cacher::Job::CacheWrite {
            text: text_arc.clone(),
            map: map_arc.clone(),
            fingerprint: fp.clone(),
            embeddings: embeddings.clone(),
        },
    );

    crate::text_cacher::CacheWriter::get().shutdown();

    let result = backend.try_load(&file_path, &fp).unwrap();

    assert!(result.is_some());
    let doc = result.unwrap();
    assert_eq!(doc.text, *text_arc);
    assert_eq!(doc.map, *map_arc);
    assert_eq!(doc.fingerprint, fp);
    let emb = doc.embeddings.expect("embeddings should be Some");
    assert_eq!(emb.len(), 3);
    assert_eq!(emb[0], vec![0.7, 0.8, 0.9]);
    assert_eq!(emb[1], vec![1.0, 1.1, 1.2]);
    assert_eq!(emb[2], vec![1.3, 1.4, 1.5]);
}
