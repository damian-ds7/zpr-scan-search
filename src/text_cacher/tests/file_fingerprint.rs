use crate::text_cacher::FileFingerprint;

#[test]
fn test_fingerprint_round_trip() {
    let fp = FileFingerprint::new_raw(1234567890, 123456789, 4096);
    let mut buf = Vec::new();
    fp.write_to(&mut buf).unwrap();
    let result = FileFingerprint::read_from(&mut buf.as_slice()).unwrap();
    assert_eq!(fp, result);
}
