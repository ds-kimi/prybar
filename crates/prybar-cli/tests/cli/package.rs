use super::fixture::{json, model, prybar, vpk};
use std::path::Path;
use tempfile::tempdir;

fn s(p: &Path) -> &str {
    p.to_str().unwrap()
}

#[test]
fn list_filters_and_truncates() {
    let tmp = tempdir().unwrap();
    let pak = vpk(tmp.path());
    let mut cmd = prybar(tmp.path());
    cmd.args(["list", s(&pak), "--limit", "1"]);
    let r = json(cmd, 0);
    assert_eq!(
        (r["format"].as_str(), r["total"].as_u64()),
        (Some("vpk"), Some(3))
    );
    assert_eq!(r["truncated"], true);
    let mut cmd = prybar(tmp.path());
    cmd.args(["list", s(&pak), "--filter", "*.vvd"]);
    assert_eq!(json(cmd, 0)["entries"][0]["path"], "models/crate.vvd");
    let out = prybar(tmp.path())
        .args(["list", s(&pak)])
        .assert()
        .success();
    let text = String::from_utf8_lossy(&out.get_output().stdout).into_owned();
    assert!(text.contains("models/crate.mdl") && text.contains("3 files"));
}

#[test]
fn pack_gma_then_unpack() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("addon");
    model(&src, "a");
    std::fs::write(src.join("addon.json"), r#"{"title":"t","type":"model"}"#).unwrap();
    let gma = tmp.path().join("a.gma");
    let mut cmd = prybar(tmp.path());
    cmd.args(["pack", s(&src), "-o", s(&gma)]);
    assert_eq!(json(cmd, 0)["files"], 3);
    let out = tmp.path().join("x");
    let mut cmd = prybar(tmp.path());
    cmd.args(["unpack", s(&gma), "-o", s(&out)]);
    assert_eq!(json(cmd, 0)["format"], "gma");
    assert!(out.join("models/a.mdl").is_file());
}

#[test]
fn pack_vpk_and_refuse_read_only_formats() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    model(&src, "a");
    let pak = tmp.path().join("p_dir.vpk");
    let mut cmd = prybar(tmp.path());
    cmd.args(["pack", s(&src), "-o", s(&pak), "--chunk-size", "100"]);
    assert_eq!(json(cmd, 0)["files"], 3);
    assert!(tmp.path().join("p_000.vpk").is_file());
    let apk = tmp.path().join("p.apk");
    prybar(tmp.path())
        .args(["pack", s(&src), "-o", s(&apk)])
        .assert()
        .code(2);
    let none = tmp.path().join("no.vpk");
    prybar(tmp.path()).args(["list", s(&none)]).assert().code(3);
}
