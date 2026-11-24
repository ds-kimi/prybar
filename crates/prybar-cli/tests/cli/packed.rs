use super::fixture::{json, prybar, vpk};
use std::path::Path;
use tempfile::tempdir;

fn s(p: &Path) -> &str {
    p.to_str().unwrap()
}

#[test]
fn decompile_from_inside_a_vpk() {
    let tmp = tempdir().unwrap();
    let pak = vpk(tmp.path());
    let out = tmp.path().join("out");
    let one = format!("{}:models/crate.mdl", s(&pak));
    let mut cmd = prybar(tmp.path());
    cmd.args(["inspect", &one]);
    assert_eq!(json(cmd, 0)["version"], 49);
    let folder = format!("{}:models", s(&pak));
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", &folder, "-o", s(&out)]);
    let r = json(cmd, 0);
    assert!(r["models"][0]["notes"].is_null(), "{r}");
    // Relative to the `models` prefix the ref named, like a disk folder.
    assert!(out.join("crate").is_dir());
}

#[test]
fn package_wildcard_depth_follows_recursive() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    super::fixture::model(&src, "top");
    super::fixture::model(&src.join("models"), "deep");
    let pak = tmp.path().join("p.vpk");
    let opts = prybar_vpk::PackOptions {
        version: 2,
        chunk_size: None,
    };
    prybar_vpk::pack(&src, &pak, &opts).unwrap();
    let glob = format!("{}:models/*.mdl", s(&pak));
    let out = tmp.path().join("out");
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", &glob, "-o", s(&out)]);
    assert_eq!(json(cmd, 0)["models"].as_array().unwrap().len(), 1);
    let mut cmd = prybar(tmp.path());
    cmd.args(["decompile", &glob, "-r", "-o", s(&out)]);
    assert_eq!(json(cmd, 0)["models"].as_array().unwrap().len(), 2);
}
