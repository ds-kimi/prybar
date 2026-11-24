use super::s;
use crate::fixture::{json, model, prybar};
use std::path::Path;
use tempfile::tempdir;

/// Gives the fixture model one `$includemodel` naming `name`, in the
/// free space at the end of the synthetic file.
fn with_include(mdl: &Path, name: &str) {
    let mut data = std::fs::read(mdl).unwrap();
    for (at, v) in [(336, 1), (340, 1700), (1700, 20), (1704, 30i32)] {
        data[at..at + 4].copy_from_slice(&v.to_le_bytes());
    }
    data[1730..1730 + name.len()].copy_from_slice(name.as_bytes());
    std::fs::write(mdl, data).unwrap();
}

#[test]
fn folders_mirror_the_tree_and_flat_numbers_clashes() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    model(&src.join("x"), "crate");
    model(&src.join("y"), "crate");
    let out = tmp.path().join("out");
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(&src), "-r", "-o", s(&out)]);
    assert!(json(cmd, 0)["warnings"].is_null());
    assert!(out.join("x/models/crate").is_dir());
    assert!(out.join("y/models/crate").is_dir());
    let flat = tmp.path().join("flat");
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(&src), "-r", "--flat", "-o", s(&flat)]);
    assert_eq!(json(cmd, 0)["warnings"].as_array().unwrap().len(), 1);
    assert!(flat.join("crate").is_dir() && flat.join("crate_2").is_dir());
}

#[test]
fn included_models_come_along_once() {
    let tmp = tempdir().unwrap();
    let male = model(tmp.path(), "male");
    let female = model(tmp.path(), "female");
    model(tmp.path(), "shared");
    with_include(&male, "models/shared.mdl");
    with_include(&female, "models/shared.mdl");
    let out = tmp.path().join("out");
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(&male), s(&female), "-o", s(&out)]);
    let r = json(cmd, 0);
    let models = r["models"].as_array().unwrap();
    assert_eq!(models.len(), 3, "{r}");
    let from = models[1]["included_from"].as_str().unwrap();
    assert!(from.ends_with("male.mdl"), "{r}");
    assert!(out.join("shared").is_dir());
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(&male), "--no-includes", "-o", s(&out)]);
    assert_eq!(json(cmd, 0)["models"].as_array().unwrap().len(), 1);
}

#[test]
fn missing_include_is_a_warning() {
    let tmp = tempdir().unwrap();
    let male = model(tmp.path(), "male");
    with_include(&male, "models/gone.mdl");
    let out = tmp.path().join("out");
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", s(&male), "-o", s(&out)]);
    let r = json(cmd, 0);
    assert_eq!(r["models"].as_array().unwrap().len(), 1);
    let w = r["warnings"][0].as_str().unwrap();
    assert!(w.contains("models/gone.mdl not found"), "{r}");
}
