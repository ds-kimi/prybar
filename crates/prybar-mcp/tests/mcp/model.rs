use super::client::{call, connect};
use super::mdl_common::{build, geometry};
use serde_json::json;
use std::path::{Path, PathBuf};

/// `<dir>/models/<name>.mdl` with its VVD and VTX.
pub fn model(dir: &Path, name: &str) -> PathBuf {
    let models = dir.join("models");
    std::fs::create_dir_all(&models).unwrap();
    std::fs::write(models.join(format!("{name}.vvd")), geometry::vvd()).unwrap();
    std::fs::write(models.join(format!("{name}.dx90.vtx")), geometry::vtx()).unwrap();
    let mdl = models.join(format!("{name}.mdl"));
    std::fs::write(&mdl, build()).unwrap();
    mdl
}

#[tokio::test]
async fn inspect_model_reports_version_and_errors() {
    let tmp = tempfile::tempdir().unwrap();
    let mdl = model(tmp.path(), "a");
    let c = connect().await;
    let (err, v) = call(&c, "inspect_model", json!({ "model": mdl })).await;
    assert!(!err, "{v}");
    assert_eq!(v["version"], 49);
    let (err, v) = call(&c, "inspect_model", json!({ "model": "models/a.mdl" })).await;
    assert!(err);
    assert_eq!(
        (v["kind"].as_str(), v["code"].as_u64()),
        (Some("usage"), Some(2))
    );
    let missing = tmp.path().join("none.mdl");
    let (_, v) = call(&c, "inspect_model", json!({ "model": missing })).await;
    assert_eq!(v["kind"], "not_found");
}

#[tokio::test]
async fn decompile_model_writes_files_and_caps_inputs() {
    let tmp = tempfile::tempdir().unwrap();
    let mdl = model(tmp.path(), "a");
    let out = tmp.path().join("out");
    let c = connect().await;
    let args = json!({ "inputs": [mdl], "out": out });
    let (err, v) = call(&c, "decompile_model", args).await;
    assert!(!err, "{v}");
    assert_eq!(
        (v["decompiled"].as_u64(), v["failed"].as_u64()),
        (Some(1), Some(0))
    );
    let qc = std::fs::read_dir(out.join("a")).unwrap();
    assert!(qc
        .flatten()
        .any(|e| e.path().extension().is_some_and(|x| x == "qc")));
    let (err, _) = call(&c, "decompile_model", json!({ "inputs": [], "out": out })).await;
    assert!(err);
    let many = vec![mdl.clone(); 65];
    let (err, v) = call(&c, "decompile_model", json!({ "inputs": many, "out": out })).await;
    assert!(err && v["error"].as_str().unwrap().contains("64"));
}

#[cfg(windows)]
#[tokio::test]
async fn unc_and_device_paths_are_refused() {
    let c = connect().await;
    for model in [
        r"\host\share\a.mdl",
        r"\?\UNC\host\share\a.mdl",
        r"\.\pipe\x",
    ] {
        let (err, v) = call(&c, "inspect_model", json!({ "model": model })).await;
        assert!(err && v["kind"] == "usage", "{model} -> {v}");
        assert!(v["error"].as_str().unwrap().contains("local"), "{v}");
    }
}
