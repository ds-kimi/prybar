use prybar_gma::{extract_all, Error, Gma};
use prybar_io::Writer;

/// Hand-built v3 addon with one entry, so tests can forge fields.
fn forged(path: &str, size: i64, crc: u32) -> Vec<u8> {
    let mut w = Writer::new();
    w.put_bytes(b"GMAD").put(3u8).put(0u64).put(0u64).put(0u8);
    w.put_cstr("n").put_cstr("{}").put_cstr("a").put(1i32);
    w.put(1u32).put_cstr(path).put(size).put(crc).put(0u32);
    w.put_bytes(b"hi").put(0u32);
    w.into_inner()
}

#[test]
fn traversal_is_rejected_before_writing() {
    let gma = Gma::from_bytes(forged("../../evil.lua", 2, crc32fast::hash(b"hi"))).unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("out");
    assert!(matches!(extract_all(&gma, &out), Err(Error::UnsafePath(_))));
    assert!(!out.exists());
}

#[test]
fn forged_fields_error_instead_of_panicking() {
    let good = forged("a.lua", 2, crc32fast::hash(b"hi"));
    for cut in 0..good.len() - 6 {
        assert!(Gma::from_bytes(good[..cut].to_vec()).is_err(), "cut {cut}");
    }
    let neg = Gma::from_bytes(forged("a.lua", -1, 0));
    assert!(matches!(neg, Err(Error::BadSize(_))));
    let huge = Gma::from_bytes(forged("a.lua", i64::MAX, 0)).unwrap();
    assert!(huge.read(&huge.entries[0]).is_err());
    let bad_crc = Gma::from_bytes(forged("a.lua", 2, 0)).unwrap();
    assert!(matches!(
        bad_crc.read(&bad_crc.entries[0]),
        Err(Error::Crc(_))
    ));
    assert!(matches!(
        Gma::from_bytes(b"GMAD\x09".to_vec()),
        Err(Error::BadVersion(9))
    ));
}

/// Crowbar issue #55: addons over 1 GiB extracted as empty files there
/// (32-bit sizes). Sizes and offsets stay 64-bit here.
#[test]
fn sizes_past_four_gib_are_kept_exact() {
    let size = 5i64 << 30;
    let gma = Gma::from_bytes(forged("big.vtf", size, 0)).unwrap();
    assert_eq!(gma.entries[0].size, size as u64);
}
