use crate::text_cacher::{FileFingerprint, read_fingerprint, write_fingerprint};

#[test]
fn test_fingerprint_round_trip() {
    let fp = FileFingerprint::new_raw(1234567890, 123456789, 4096);
    let mut buf = Vec::new();
    write_fingerprint(&fp, &mut buf).unwrap();
    let result = read_fingerprint(&mut buf.as_slice()).unwrap();
    assert_eq!(fp, result);
}
