#[cfg(unix)]
mod compile;
mod tree;

use crate::fixture::{json, model, prybar};
use std::path::Path;
use tempfile::tempdir;

fn s(p: &Path) -> &str {
    p.to_str().unwrap()
}

#[test]
fn all_failed_batch_keeps_the_shared_code() {
    let tmp = tempdir().unwrap();
    let out = tmp.path().join("out");
    let (a, b) = (tmp.path().join("a.mdl"), tmp.path().join("b.mdl"));
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(&a), s(&b), "-o", s(&out)]);
    assert_eq!(json(cmd, 3)["failures"].as_array().unwrap().len(), 2);
    std::fs::write(&a, b"junk").unwrap();
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(&a), s(&b), "-o", s(&out)]);
    assert_eq!(json(cmd, 4)["models"].as_array().unwrap().len(), 0);
}

#[test]
fn same_name_in_two_folders_gets_two_outputs() {
    let tmp = tempdir().unwrap();
    let a = model(&tmp.path().join("x"), "Crate");
    let b = model(&tmp.path().join("y"), "crate");
    let out = tmp.path().join("out");
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(&a), s(&b), "-o", s(&out)]);
    let r = json(cmd, 0);
    let dir = |i: usize| r["models"][i]["out_dir"].as_str().unwrap().to_lowercase();
    assert_ne!(dir(0), dir(1));
}

#[test]
fn upper_case_extensions_are_found() {
    let tmp = tempdir().unwrap();
    let mdl = model(tmp.path(), "a");
    let models = mdl.parent().unwrap();
    for (from, to) in [
        ("a.mdl", "A.MDL"),
        ("a.vvd", "A.VVD"),
        ("a.dx90.vtx", "A.DX90.VTX"),
    ] {
        std::fs::rename(models.join(from), models.join(to)).unwrap();
    }
    let out = tmp.path().join("out");
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(&models.join("A.MDL")), "-o", s(&out)]);
    assert!(json(cmd, 0)["models"][0]["notes"].is_null());
    let mut cmd = prybar(tmp.path());
    cmd.args(["pack", s(tmp.path()), "-o", s(&tmp.path().join("p.vpk"))]);
    cmd.args(["--vpk-version", "3"]);
    assert_eq!(json(cmd, 2)["error"]["code"], 2);
}
