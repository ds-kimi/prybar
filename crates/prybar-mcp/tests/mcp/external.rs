use super::client::{call, connect};
use super::model::model;
use serde_json::json;

#[tokio::test]
async fn compile_and_view_need_a_configured_game() {
    let tmp = tempfile::tempdir().unwrap();
    let qc = tmp.path().join("a.qc");
    std::fs::write(&qc, "$modelname a.mdl\n").unwrap();
    let mdl = model(tmp.path(), "a");
    let c = connect().await;
    let args = json!({ "game": "no-such-game", "qc": qc });
    let (err, v) = call(&c, "compile_model", args).await;
    assert!(err && v["kind"] == "usage", "{v}");
    let args = json!({ "game": "no-such-game", "model": mdl });
    let (err, v) = call(&c, "view_model", args).await;
    assert!(err && v["kind"] == "usage", "{v}");
    let (err, v) = call(&c, "compile_model", json!({ "game": "g", "qc": "a.qc" })).await;
    assert!(err && v["error"].as_str().unwrap().contains("absolute"));
}

#[tokio::test]
async fn publish_workshop_checks_its_spec() {
    let tmp = tempfile::tempdir().unwrap();
    let c = connect().await;
    let (err, v) = call(&c, "publish_workshop", json!({})).await;
    assert!(err && v["kind"] == "usage", "{v}");
    let spec = tmp.path().join("item.json");
    std::fs::write(&spec, r#"{"app_id": 4000, "content": "rel/addon.gma"}"#).unwrap();
    let args = json!({ "spec": spec, "dry_run": true });
    let (err, v) = call(&c, "publish_workshop", args).await;
    assert!(
        err && v["error"].as_str().unwrap().contains("content"),
        "{v}"
    );
    std::fs::write(&spec, "not json").unwrap();
    let (_, v) = call(&c, "publish_workshop", json!({ "spec": spec })).await;
    assert_eq!(v["kind"], "invalid_input");
}
