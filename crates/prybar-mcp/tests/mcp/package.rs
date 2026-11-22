use super::client::{call, connect};
use super::model::model;
use serde_json::json;

#[tokio::test]
async fn pack_list_unpack_round_trip() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    model(&src, "crate");
    std::fs::write(src.join("addon.json"), r#"{"title":"t","type":"model"}"#).unwrap();
    let gma = tmp.path().join("addon.gma");
    let c = connect().await;
    let (err, v) = call(&c, "pack_package", json!({ "src": src, "out": gma })).await;
    assert!(!err, "{v}");
    assert_eq!(v["files"], 3);
    let args = json!({ "package": gma, "filter": "*.mdl", "limit": 10 });
    let (err, v) = call(&c, "list_package", args).await;
    assert!(!err, "{v}");
    assert_eq!(
        (v["total"].as_u64(), v["truncated"].as_bool()),
        (Some(1), Some(false))
    );
    let out = tmp.path().join("out");
    let (err, v) = call(&c, "unpack_package", json!({ "package": gma, "out": out })).await;
    assert!(!err, "{v}");
    assert!(out.join("models").join("crate.mdl").is_file());
}

#[tokio::test]
async fn list_package_caps_entries_and_checks_paths() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    model(&src, "crate");
    std::fs::write(src.join("addon.json"), r#"{"title":"t","type":"model"}"#).unwrap();
    let gma = tmp.path().join("addon.gma");
    let c = connect().await;
    call(&c, "pack_package", json!({ "src": src, "out": gma })).await;
    let (_, v) = call(&c, "list_package", json!({ "package": gma, "limit": 0 })).await;
    assert_eq!(v["entries"].as_array().unwrap().len(), 1);
    assert_eq!(
        (v["total"].as_u64(), v["truncated"].as_bool()),
        (Some(3), Some(true))
    );
    let (err, v) = call(
        &c,
        "unpack_package",
        json!({ "package": gma, "out": "rel" }),
    )
    .await;
    assert!(err && v["error"].as_str().unwrap().contains("out"));
    let vpk = json!({ "src": src, "out": tmp.path().join("x.vpk"), "vpk_version": 3 });
    let (err, v) = call(&c, "pack_package", vpk).await;
    assert!(err && v["kind"] == "usage", "{v}");
}

#[tokio::test]
async fn pack_gma_without_addon_json_names_the_file() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    model(&src, "crate");
    let c = connect().await;
    let args = json!({ "src": src, "out": tmp.path().join("a.gma") });
    let (err, v) = call(&c, "pack_package", args).await;
    assert!(err && v["kind"] == "usage", "{v}");
    assert!(v["error"].as_str().unwrap().contains("addon.json"));
}
