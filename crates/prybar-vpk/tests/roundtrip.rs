mod common;

use prybar_vpk::{archive_path, extract_all, pack, PackOptions, Vpk};
use std::path::Path;

fn roundtrip(opts: PackOptions, name: &str) -> Vpk {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    let out = tmp.path().join("out");
    common::make_tree(&src);
    let dir = tmp.path().join(name);
    assert_eq!(pack(&src, &dir, &opts).unwrap(), 7);
    let vpk = Vpk::open(&dir).unwrap();
    assert_eq!(vpk.header.version, opts.version);
    assert_eq!(extract_all(&vpk, &out).unwrap(), 7);
    assert_eq!(common::snapshot(&src), common::snapshot(&out));
    vpk
}

#[test]
fn v1_single_file_roundtrip() {
    let vpk = roundtrip(
        PackOptions {
            version: 1,
            chunk_size: None,
        },
        "one.vpk",
    );
    assert!(vpk.entries.iter().all(|e| e.is_embedded()));
    let mut paths: Vec<_> = vpk.entries.iter().map(|e| e.path.as_str()).collect();
    paths.sort_unstable();
    assert_eq!(paths[0], "cfg/.hidden");
    assert!(paths.contains(&"noext"));
}

#[test]
fn v2_multi_archive_roundtrip() {
    let opts = PackOptions {
        version: 2,
        chunk_size: Some(64),
    };
    let vpk = roundtrip(opts, "multi_dir.vpk");
    let max = vpk.entries.iter().map(|e| e.archive_index).max().unwrap();
    assert!(max >= 2, "small chunks must split into several archives");
}

#[test]
fn multi_archive_requires_dir_suffix() {
    let tmp = tempfile::tempdir().unwrap();
    let opts = PackOptions {
        version: 1,
        chunk_size: Some(10),
    };
    assert!(pack(tmp.path(), &tmp.path().join("x.vpk"), &opts).is_err());
    let dir = Path::new("pak01_dir.vpk");
    assert_eq!(archive_path(dir, 12).unwrap(), Path::new("pak01_012.vpk"));
    assert!(archive_path(dir, 0x7fff).is_none());
}
