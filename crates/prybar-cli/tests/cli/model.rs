use super::fixture::{json, model, prybar};
use std::path::Path;
use tempfile::tempdir;

fn s(p: &Path) -> &str {
    p.to_str().unwrap()
}

#[test]
fn inspect_reports_version_and_codes() {
    let tmp = tempdir().unwrap();
    let mdl = model(tmp.path(), "a");
    let mut cmd = prybar(tmp.path());
    cmd.args(["inspect", s(&mdl)]);
    assert_eq!(json(cmd, 0)["version"], 49);
    let mut cmd = prybar(tmp.path());
    cmd.args(["inspect", s(&tmp.path().join("none.mdl"))]);
    assert_eq!(json(cmd, 3)["error"]["code"], 3);
    let junk = tmp.path().join("junk.mdl");
    std::fs::write(&junk, b"not a model").unwrap();
    prybar(tmp.path())
        .args(["inspect", s(&junk)])
        .assert()
        .code(4);
}

#[test]
fn decompile_file_writes_qc_and_smd() {
    let tmp = tempdir().unwrap();
    let mdl = model(tmp.path(), "a");
    let out = tmp.path().join("out");
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(&mdl), "-o", s(&out)]);
    let r = json(cmd, 0);
    assert_eq!(r["models"].as_array().unwrap().len(), 1);
    let has = |ext: &str| {
        std::fs::read_dir(out.join("a"))
            .unwrap()
            .any(|e| e.unwrap().path().extension().is_some_and(|x| x == ext))
    };
    assert!(has("qc") && has("smd"));
}

#[test]
fn decompile_batch_glob_and_partial_failure() {
    let tmp = tempdir().unwrap();
    model(tmp.path(), "a");
    model(&tmp.path().join("sub"), "b");
    std::fs::write(tmp.path().join("models/bad.mdl"), b"IDST").unwrap();
    let out = tmp.path().join("out");
    let glob = tmp.path().join("models").join("*.mdl");
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(&glob), "-o", s(&out)]);
    let r = json(cmd, 7);
    assert_eq!(r["models"].as_array().unwrap().len(), 1);
    assert_eq!(r["failures"][0]["code"], 4);
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(tmp.path()), "-r", "-o", s(&out), "--quiet"]);
    assert_eq!(json(cmd, 7)["models"].as_array().unwrap().len(), 2);
}
