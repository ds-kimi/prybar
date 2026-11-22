use prybar_gma::{extract_all, is_whitelisted, pack, Error, Gma};
use std::path::Path;

fn write(root: &Path, rel: &str, data: &[u8]) {
    let p = root.join(rel);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, data).unwrap();
}

const ADDON_JSON: &str =
    r#"{"title":"Test","type":"tool","tags":["fun"],"ignore":["*.psd"],"description":"d"}"#;

#[test]
fn pack_list_unpack_roundtrip() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    write(&src, "addon.json", ADDON_JSON.as_bytes());
    write(&src, "lua/autorun/a.lua", b"print('hi')");
    write(&src, "Materials/X.vmt", b"\"VertexLitGeneric\" {}");
    write(&src, "models/empty.mdl", b"");
    write(&src, "materials/src.psd", b"ignored");
    let out = tmp.path().join("a.gma");
    assert_eq!(pack(&src, &out).unwrap(), 3);
    let gma = Gma::open(&out).unwrap();
    assert_eq!(gma.header.name, "Test");
    assert!(gma.header.description.contains("\"type\":\"tool\""));
    let paths: Vec<_> = gma.entries.iter().map(|e| e.path.as_str()).collect();
    assert_eq!(
        paths,
        ["lua/autorun/a.lua", "materials/x.vmt", "models/empty.mdl"]
    );
    let dst = tmp.path().join("dst");
    assert_eq!(extract_all(&gma, &dst).unwrap(), 3);
    let lua = std::fs::read(dst.join("lua/autorun/a.lua")).unwrap();
    assert_eq!(lua, b"print('hi')");
}

#[test]
fn pack_rejects_non_whitelisted_files() {
    let tmp = tempfile::tempdir().unwrap();
    write(tmp.path(), "addon.json", ADDON_JSON.as_bytes());
    write(tmp.path(), "lua/ok.lua", b"");
    write(tmp.path(), "bin/evil.dll", b"MZ");
    write(tmp.path(), "lua/notes.exe", b"");
    let out = tmp.path().join("a.gma");
    match pack(tmp.path(), &out) {
        Err(Error::NotWhitelisted(bad)) => assert_eq!(bad, ["bin/evil.dll", "lua/notes.exe"]),
        other => panic!("expected whitelist error, got {other:?}"),
    }
    assert!(!out.exists());
}

#[test]
fn whitelist_globs_span_folders() {
    assert!(is_whitelisted("lua/a/b/c.lua"));
    assert!(is_whitelisted("gamemodes/x/gamemode/init.lua"));
    assert!(!is_whitelisted("lua/a.luac"));
    assert!(!is_whitelisted("addon.json"));
}
