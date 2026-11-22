mod support;

use prybar_hfs::{extract_all, Error, Hfs};
use support::build;

#[test]
fn lists_and_extracts_plain_skips_comp() {
    let pkg = build(&[
        ("a.txt", b"hello"),
        (r"sub\b.bin", b"\x01\x02"),
        ("c.mdl.comp", b"zz"),
    ]);
    let hfs = Hfs::from_bytes(pkg).unwrap();
    let paths: Vec<_> = hfs.entries.iter().map(|e| e.path.as_str()).collect();
    assert_eq!(paths, ["a.txt", "sub/b.bin", "c.mdl.comp"]);
    let tmp = tempfile::tempdir().unwrap();
    assert_eq!(extract_all(&hfs, tmp.path()).unwrap(), 2);
    assert_eq!(
        std::fs::read(tmp.path().join("sub/b.bin")).unwrap(),
        b"\x01\x02"
    );
    assert!(!tmp.path().join("c.mdl.comp").exists());
    assert!(matches!(
        hfs.read(&hfs.entries[2]),
        Err(Error::Unsupported(_))
    ));
}

#[test]
fn hostile_input_errors_instead_of_panicking() {
    let good = build(&[("a.txt", b"hi")]);
    for cut in 0..good.len() {
        assert!(Hfs::from_bytes(good[..cut].to_vec()).is_err(), "cut {cut}");
    }
    let n = good.len();
    let mut forged = good.clone();
    forged[n - 12..n - 10].copy_from_slice(&u16::MAX.to_le_bytes());
    assert!(Hfs::from_bytes(forged).is_err());
    let mut bad_dir = good.clone();
    bad_dir[n - 6..n - 2].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(Hfs::from_bytes(bad_dir).is_err());
    let mut oob = Hfs::from_bytes(good).unwrap();
    oob.entries[0].stored_size = u32::MAX;
    assert!(oob.read(&oob.entries[0]).is_err());
    assert!(matches!(
        Hfs::from_bytes(vec![0; 64]),
        Err(Error::NoDirectory)
    ));
}

#[test]
fn extract_is_all_or_nothing_and_skips_dirs() {
    let evil = Hfs::from_bytes(build(&[(r"..\..\x.txt", b"hi")])).unwrap();
    let tmp = tempfile::tempdir().unwrap();
    assert!(matches!(
        extract_all(&evil, tmp.path()),
        Err(Error::UnsafePath(_))
    ));
    let mut partial = Hfs::from_bytes(build(&[("a.txt", b"hi"), ("dir/", b"")])).unwrap();
    partial.entries.push(partial.entries[0].clone());
    partial.entries[2].path = "late.txt".into();
    partial.entries[2].stored_size = u32::MAX;
    assert!(extract_all(&partial, tmp.path()).is_err());
    assert!(!tmp.path().join("a.txt").exists());
    partial.entries.pop();
    assert_eq!(extract_all(&partial, tmp.path()).unwrap(), 1);
}
