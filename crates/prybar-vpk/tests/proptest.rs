//! Writer round-trip: any tree packed then read back yields the same bytes.

use proptest::prelude::*;
use prybar_vpk::{pack, PackOptions, Vpk};
use std::collections::BTreeMap;

// Lowercase only: VPK paths are case-folded by the game, so mixed case
// would make two generated names collide on Windows.
fn files() -> impl Strategy<Value = BTreeMap<String, Vec<u8>>> {
    let path = r"[a-z]{1,6}(/[a-z]{1,6}){0,2}\.[a-z]{1,3}";
    prop::collection::btree_map(path, prop::collection::vec(any::<u8>(), 0..300), 1..12)
}

fn roundtrip(files: &BTreeMap<String, Vec<u8>>, opts: PackOptions, name: &str) {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    for (rel, data) in files {
        let p = src.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, data).unwrap();
    }
    let out = tmp.path().join(name);
    pack(&src, &out, &opts).unwrap();
    let vpk = Vpk::open(&out).unwrap();
    let read: BTreeMap<String, Vec<u8>> = vpk
        .entries
        .iter()
        .map(|e| (e.path.clone(), vpk.read(e).unwrap()))
        .collect();
    assert_eq!(&read, files);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]

    #[test]
    fn pack_then_read_is_identity(files in files(), version in 1u32..=2, split in any::<bool>()) {
        let chunk_size = split.then_some(256);
        roundtrip(&files, PackOptions { version, chunk_size }, "p_dir.vpk");
    }
}
