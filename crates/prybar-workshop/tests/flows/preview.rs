use prybar_workshop::preview::{prepare_preview, PREVIEW_MAX};

use super::fixture::png;

#[test]
fn stills_become_512_squares_under_the_limit() {
    let tmp = tempfile::tempdir().unwrap();
    for (w, h) in [(300, 200), (2000, 900), (1, 1), (512, 512)] {
        let out = prepare_preview(&png(tmp.path(), w, h), tmp.path()).unwrap();
        let img = image::open(&out).unwrap();
        assert_eq!((img.width(), img.height()), (512, 512));
        assert!(std::fs::metadata(&out).unwrap().len() < PREVIEW_MAX);
    }
}

#[test]
fn garbage_and_huge_canvases_fail_cleanly() {
    let tmp = tempfile::tempdir().unwrap();
    let junk = tmp.path().join("junk.png");
    std::fs::write(&junk, b"\x89PNG\r\n\x1a\nnot really").unwrap();
    assert!(prepare_preview(&junk, tmp.path()).is_err());
    // A header claiming 100000x100000 must hit the limits, not allocate.
    let mut big = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR".to_vec();
    big.extend(100_000u32.to_be_bytes());
    big.extend(100_000u32.to_be_bytes());
    big.extend([8, 2, 0, 0, 0, 0, 0, 0, 0]);
    let big_path = tmp.path().join("big.png");
    std::fs::write(&big_path, big).unwrap();
    assert!(prepare_preview(&big_path, tmp.path()).is_err());
    assert!(prepare_preview(&tmp.path().join("none.png"), tmp.path()).is_err());
}
