use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::tempdir;

use crate::cacher::{
    CacheBackend, CacheWriter, Embeddings, FileFingerprint, GlobalCache, Job, WordMap,
};

#[test]
fn test_global_cache_round_trip() {
    let cache_root = tempdir().unwrap();
    let backend = GlobalCache::new(cache_root.path().to_path_buf());

    let doc_path = PathBuf::from("/home/user/test.pdf");
    let fp = FileFingerprint {
        mtime_secs: 1000,
        mtime_nanos: 2000,
        size: 3000,
    };

    let text: Arc<str> = Arc::from("Hello world".to_string().into_boxed_str());
    let mut map = WordMap::new();
    map.insert("Hello".to_string(), vec![0]);
    map.insert("world".to_string(), vec![1]);
    let map_arc = Arc::new(map);
    let embeddings = Arc::new(Some(Embeddings::from(vec![vec![0.1, 0.2]])));

    backend.submit_job(
        doc_path.clone(),
        Job::CacheWrite {
            text: text.clone(),
            map: map_arc.clone(),
            fingerprint: fp.clone(),
            embeddings: embeddings.clone(),
        },
    );

    CacheWriter::get().shutdown();

    let result = backend.try_load(&doc_path, &fp, false).unwrap();
    assert!(result.is_some());
    let doc = result.unwrap();
    assert_eq!(doc.text, "Hello world");
    assert_eq!(doc.map.get("Hello").unwrap(), &vec![0]);
    assert_eq!(doc.fingerprint, fp);
    assert!(doc.embeddings.is_some());

    let cache_dir_name = format!("{}-{}-{}", fp.mtime_secs, fp.mtime_nanos, fp.size);
    let last_used_path = cache_root.path().join(cache_dir_name).join(".lastused");
    assert!(last_used_path.exists());
}

#[test]
fn test_global_cache_hit_after_move() {
    let cache_root = tempdir().unwrap();
    let backend = GlobalCache::new(cache_root.path().to_path_buf());

    let fp = FileFingerprint {
        mtime_secs: 555,
        mtime_nanos: 666,
        size: 777,
    };

    let doc_path_1 = PathBuf::from("/dir1/document.pdf");
    let text: Arc<str> = Arc::from("content".to_string().into_boxed_str());
    let map = Arc::new(WordMap::new());
    let embeddings = Arc::new(None);

    backend.submit_job(
        doc_path_1.clone(),
        Job::CacheWrite {
            text: text.clone(),
            map: map.clone(),
            fingerprint: fp.clone(),
            embeddings: embeddings.clone(),
        },
    );

    CacheWriter::get().shutdown();

    let doc_path_2 = PathBuf::from("/completely/different/place/document.pdf");
    let result = backend.try_load(&doc_path_2, &fp, false).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().text, "content");
}

#[test]
fn test_global_cache_partial_missing() {
    let cache_root = tempdir().unwrap();
    let backend = GlobalCache::new(cache_root.path().to_path_buf());

    let fp = FileFingerprint {
        mtime_secs: 1,
        mtime_nanos: 1,
        size: 1,
    };
    let doc_path = PathBuf::from("test.txt");

    backend.submit_job(
        doc_path.clone(),
        Job::CacheWrite {
            text: Arc::from("text".to_string().into_boxed_str()),
            map: Arc::new(WordMap::new()),
            fingerprint: fp.clone(),
            embeddings: Arc::new(None),
        },
    );

    CacheWriter::get().shutdown();

    let cache_dir = cache_root.path().join("1-1-1");
    let map_path = cache_dir.join("test.txt.map");
    fs::remove_file(map_path).unwrap();

    let result = backend.try_load(&doc_path, &fp, false).unwrap();
    assert!(result.is_none(), "Should miss if map file is missing");
}

#[test]
fn test_global_cache_cleanup_invalid_data() {
    let cache_root = tempdir().unwrap();
    let backend = GlobalCache::new(cache_root.path().to_path_buf());

    let cache_dir = cache_root.path().join("garbage-cache");
    fs::create_dir_all(&cache_dir).unwrap();
    fs::write(cache_dir.join(".lastused"), "not-a-number").unwrap();

    backend.cleanup_unused(1).unwrap();
    assert!(cache_dir.exists());
}

#[test]
fn test_global_cache_cleanup() {
    let cache_root = tempdir().unwrap();
    let backend = GlobalCache::new(cache_root.path().to_path_buf());

    let cache_dir_old = cache_root.path().join("1000-0-100");
    fs::create_dir_all(&cache_dir_old).unwrap();

    let old_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - (10 * 24 * 60 * 60); // 10 days ago
    fs::write(cache_dir_old.join(".lastused"), old_time.to_string()).unwrap();

    let cache_dir_new = cache_root.path().join("2000-0-200");
    fs::create_dir_all(&cache_dir_new).unwrap();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    fs::write(cache_dir_new.join(".lastused"), now.to_string()).unwrap();

    backend.cleanup_unused(5).unwrap();

    assert!(!cache_dir_old.exists());
    assert!(cache_dir_new.exists());
}

#[test]
fn test_global_cache_reload_true() {
    let cache_root = tempdir().unwrap();
    let backend = GlobalCache::new(cache_root.path().to_path_buf());

    let doc_path = PathBuf::from("reload_test.pdf");
    let fp = FileFingerprint {
        mtime_secs: 123,
        mtime_nanos: 456,
        size: 789,
    };

    backend.submit_job(
        doc_path.clone(),
        Job::CacheWrite {
            text: Arc::from("content".to_string().into_boxed_str()),
            map: Arc::new(WordMap::new()),
            fingerprint: fp.clone(),
            embeddings: Arc::new(None),
        },
    );
    CacheWriter::get().shutdown();

    let result = backend.try_load(&doc_path, &fp, true).unwrap();
    assert!(
        result.is_none(),
        "Should return None when reload_cache is true"
    );

    let result = backend.try_load(&doc_path, &fp, false).unwrap();
    assert!(
        result.is_some(),
        "Should return Some when reload_cache is false"
    );
}
