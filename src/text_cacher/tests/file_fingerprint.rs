use crate::text_cacher::{
    FileFingerprint,
    codec::{read_fingerprint, write_fingerprint},
};

#[test]
fn test_fingerprint_round_trip() {
    let fp = FileFingerprint {
        mtime_secs: 1234,
        mtime_nanos: 5678,
        size: 999,
    };
    let mut buf = Vec::new();
    write_fingerprint(&fp, &mut buf).unwrap();
    let result = read_fingerprint(&mut buf.as_slice()).unwrap();
    assert_eq!(fp, result);
}
