//! Writer round-trip: any whitelisted tree packed then read back yields
//! the same bytes under the same (lowercased) paths.

use proptest::prelude::*;
use prybar_gma::{pack, Gma};
use std::collections::BTreeMap;

fn files() -> impl Strategy<Value = BTreeMap<String, Vec<u8>>> {
    let path = r"lua/[a-z]{1,6}(/[a-z]{1,6}){0,2}\.lua";
    prop::collection::btree_map(path, prop::collection::vec(any::<u8>(), 0..300), 1..12)
}

fn roundtrip(files: &BTreeMap<String, Vec<u8>>, title: &str) {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    let json = serde_json::json!({ "title": title, "type": "tool", "tags": ["fun"] });
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("addon.json"), json.to_string()).unwrap();
    for (rel, data) in files {
        let p = src.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, data).unwrap();
    }
    let out = tmp.path().join("a.gma");
    pack(&src, &out).unwrap();
    let gma = Gma::open(&out).unwrap();
    assert_eq!(gma.header.name, title);
    let read: BTreeMap<String, Vec<u8>> = gma
        .entries
        .iter()
        .map(|e| (e.path.clone(), gma.read(e).unwrap().to_vec()))
        .collect();
    assert_eq!(&read, files);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]

    #[test]
    fn pack_then_read_is_identity(files in files(), title in "[ -~]{0,40}") {
        roundtrip(&files, &title);
    }
}
