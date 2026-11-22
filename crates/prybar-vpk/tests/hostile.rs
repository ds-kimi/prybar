use prybar_io::Writer;
use prybar_vpk::{extract_all, Error, Vpk};

/// Hand-built v1 file with one embedded entry, so tests can forge fields.
fn forged(dir: &str, crc: u32, length: u32) -> Vec<u8> {
    let mut tree = Writer::new();
    tree.put_cstr("txt").put_cstr(dir).put_cstr("evil");
    tree.put(crc).put(0u16).put(0x7fffu16).put(0u32).put(length);
    tree.put(0xffffu16).put(0u8).put(0u8).put(0u8);
    let tree = tree.into_inner();
    let mut w = Writer::new();
    w.put(0x55aa_1234u32).put(1u32).put(tree.len() as u32);
    w.put_bytes(&tree).put_bytes(b"hi");
    w.into_inner()
}

fn open_bytes(data: &[u8]) -> (tempfile::TempDir, Result<Vpk, Error>) {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("x.vpk");
    std::fs::write(&path, data).unwrap();
    let vpk = Vpk::open(&path);
    (tmp, vpk)
}

#[test]
fn traversal_is_rejected_before_writing() {
    let (tmp, vpk) = open_bytes(&forged("../..", crc32fast::hash(b"hi"), 2));
    let vpk = vpk.unwrap();
    assert_eq!(vpk.entries[0].path, "../../evil.txt");
    let out = tmp.path().join("out");
    assert!(matches!(extract_all(&vpk, &out), Err(Error::UnsafePath(_))));
    assert!(!out.exists());
}

#[test]
fn forged_fields_error_instead_of_panicking() {
    let good = forged("d", crc32fast::hash(b"hi"), 2);
    for cut in 0..good.len() - 2 {
        let _ = open_bytes(&good[..cut]).1.map(|_| ());
    }
    let (_t, vpk) = open_bytes(&forged("d", 0, 2));
    let vpk = vpk.unwrap();
    assert!(matches!(vpk.read(&vpk.entries[0]), Err(Error::Crc(_))));
    let (_t, vpk) = open_bytes(&forged("d", 0, u32::MAX));
    let vpk = vpk.unwrap();
    assert!(matches!(vpk.read(&vpk.entries[0]), Err(Error::Format(_))));
    let (_t, bad) = open_bytes(b"\x34\x12\xaa\x55\x09\0\0\0\0\0\0\0");
    assert!(matches!(bad, Err(Error::BadVersion(9))));
}
