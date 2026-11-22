use prybar_apk::{extract_all, Apk, Error, APK_ID};
use prybar_io::Writer;

/// Synthetic package: two files, the second in a `\`-separated folder.
fn build(names: &[&str], blobs: &[&[u8]]) -> Vec<u8> {
    let mut w = Writer::new();
    w.put(APK_ID).put(16u32).put(names.len() as u32).put(0u32);
    let mut offsets = Vec::new();
    for b in blobs {
        offsets.push(w.position() as u32);
        w.put_bytes(b);
    }
    let dir = w.position() as u32;
    w.patch_u32(12, dir);
    for (i, name) in names.iter().enumerate() {
        w.put(name.len() as u32).put_cstr(name);
        w.put(offsets[i])
            .put(blobs[i].len() as u32)
            .put(0u32)
            .put(0u32);
    }
    w.into_inner()
}

#[test]
fn lists_and_extracts() {
    let apk = Apk::from_bytes(build(&["a.txt", r"sub\b.bin"], &[b"hello", b"\x01\x02"])).unwrap();
    let paths: Vec<_> = apk.entries.iter().map(|e| e.path.as_str()).collect();
    assert_eq!(paths, ["a.txt", "sub/b.bin"]);
    let tmp = tempfile::tempdir().unwrap();
    assert_eq!(extract_all(&apk, tmp.path()).unwrap(), 2);
    assert_eq!(
        std::fs::read(tmp.path().join("sub/b.bin")).unwrap(),
        b"\x01\x02"
    );
}

#[test]
fn hostile_input_errors_instead_of_panicking() {
    let good = build(&["a.txt"], &[b"hi"]);
    for cut in 0..good.len() {
        assert!(Apk::from_bytes(good[..cut].to_vec()).is_err(), "cut {cut}");
    }
    let mut forged = good.clone();
    forged[8..12].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(Apk::from_bytes(forged).is_err());
    let evil = Apk::from_bytes(build(&[r"..\..\x.txt"], &[b"hi"])).unwrap();
    let tmp = tempfile::tempdir().unwrap();
    assert!(matches!(
        extract_all(&evil, tmp.path()),
        Err(Error::UnsafePath(_))
    ));
    let mut oob = Apk::from_bytes(good).unwrap();
    oob.entries[0].size = u32::MAX;
    assert!(oob.read(&oob.entries[0]).is_err());
    assert!(matches!(Apk::from_bytes(vec![0; 16]), Err(Error::BadMagic)));
}
