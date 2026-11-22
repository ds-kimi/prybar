use prybar_io::Writer;
use std::path::Path;

fn tree(root: &Path, files: &[(&str, &[u8])]) {
    for (rel, data) in files {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().expect("parent")).expect("mkdir");
        std::fs::write(p, data).expect("write");
    }
}

/// One v1 and one v2 dir file, data embedded so the seeds stand alone.
pub fn vpks() -> Vec<(String, Vec<u8>)> {
    let tmp = tempfile::tempdir().expect("tempdir");
    let src = tmp.path().join("src");
    tree(&src, &[("a/b.txt", b"hello"), ("c.bin", b"\x00\x01\x02")]);
    (1..=2)
        .map(|version| {
            let out = tmp.path().join(format!("v{version}.vpk"));
            let opts = prybar_vpk::PackOptions {
                version,
                chunk_size: None,
            };
            prybar_vpk::pack(&src, &out, &opts).expect("pack vpk");
            (format!("v{version}.vpk"), std::fs::read(out).expect("read"))
        })
        .collect()
}

pub fn gma() -> Vec<u8> {
    let tmp = tempfile::tempdir().expect("tempdir");
    let json = br#"{"title":"Seed","type":"tool","tags":["fun"]}"#;
    tree(
        tmp.path(),
        &[("addon.json", json), ("lua/autorun/a.lua", b"print(1)")],
    );
    let out = tmp.path().join("a.gma");
    prybar_gma::pack(tmp.path(), &out).expect("pack gma");
    std::fs::read(out).expect("read")
}

/// Same layout as the APK integration tests: header, data, directory.
pub fn apk() -> Vec<u8> {
    let mut w = Writer::new();
    w.put(prybar_apk::APK_ID).put(16u32).put(1u32).put(0u32);
    w.put_bytes(b"hello");
    let dir = w.position() as u32;
    w.patch_u32(12, dir);
    w.put(5u32).put_cstr("a.txt");
    w.put(16u32).put(5u32).put(0u32).put(0u32);
    w.into_inner()
}
