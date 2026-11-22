use proptest::prelude::*;
use prybar_tools::paths::library_paths;
use std::path::PathBuf;

#[test]
fn parses_new_layout_and_ignores_old() {
    let new = br#""libraryfolders" { "0" { "path" "D:\\Games\\Steam" "apps" { "4000" "1" } } "1" { "path" "/mnt/games" } }"#;
    let want = vec![
        PathBuf::from(r"D:\Games\Steam"),
        PathBuf::from("/mnt/games"),
    ];
    assert_eq!(library_paths(new), want);
    let old = br#""LibraryFolders" { "TimeNextStatsReport" "1" "1" "/d/lib" }"#;
    assert_eq!(library_paths(old), vec![PathBuf::from("/d/lib")]);
}

#[test]
fn path_as_value_is_not_a_key() {
    assert!(library_paths(br#""label" "path" "x" "y""#).is_empty());
    let got = library_paths(br#""path" "path" "/lib""#);
    assert_eq!(got, vec![PathBuf::from("path")]);
}

#[test]
fn hostile_inputs_do_not_panic() {
    for bad in [
        &b""[..],
        b"\"",
        b"\"path\"",
        b"\"path\" \"",
        b"\xff\xfe\"path\" \"x\"",
    ] {
        let _ = library_paths(bad);
    }
    let many = "\"path\" \"/x\" ".repeat(10_000);
    assert_eq!(library_paths(many.as_bytes()).len(), 256);
}

proptest! {
    #[test]
    fn never_panics(bytes in proptest::collection::vec(any::<u8>(), 0..4096)) {
        let _ = library_paths(&bytes);
    }
}
