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
    let (_, v) = call(&c, "publish_workshop", json!({ "spec": spec })).await;
    assert_eq!(v["kind"], "usage", "a real publish without confirm: {v}");
    std::fs::write(&spec, "not json").unwrap();
    let args = json!({ "spec": spec, "dry_run": true });
    let (_, v) = call(&c, "publish_workshop", args).await;
    assert_eq!(v["kind"], "invalid_input");
}

#[tokio::test]
async fn publish_workshop_lists_games_and_resolves_game_names() {
    let tmp = tempfile::tempdir().unwrap();
    let c = connect().await;
    let (err, v) = call(&c, "publish_workshop", json!({ "list_games": true })).await;
    assert!(!err, "{v}");
    assert_eq!(v["supported"][0]["app_id"], 4000);
    assert!(v["unsupported"]
        .as_array()
        .unwrap()
        .iter()
        .any(|g| g["id"] == "l4d2"));
    let (err, v) = call(&c, "publish_workshop", json!({ "game": "l4d2" })).await;
    assert!(
        err && v["error"].as_str().unwrap().contains("legacy"),
        "{v}"
    );
    let map = tmp.path().join("map");
    std::fs::create_dir_all(&map).unwrap();
    std::fs::write(map.join("ctf_crate.bsp"), b"VBSP").unwrap();
    let spec = tmp.path().join("item.json");
    let body = json!({ "app_id": 4000, "title": "Crate", "content": map });
    std::fs::write(&spec, body.to_string()).unwrap();
    let args = json!({ "spec": spec, "game": "tf2", "dry_run": true });
    let (err, v) = call(&c, "publish_workshop", args).await;
    assert!(!err && v["dry_run"] == true, "{v}");
}

#[tokio::test]
async fn compile_model_batch_checks_its_inputs() {
    let tmp = tempfile::tempdir().unwrap();
    let c = connect().await;
    let (err, v) = call(&c, "compile_model", json!({ "game": "g" })).await;
    assert!(err && v["kind"] == "usage", "{v}");
    let args = json!({ "game": "g", "inputs": ["rel/*.qc"] });
    let (err, v) = call(&c, "compile_model", args).await;
    assert!(err && v["error"].as_str().unwrap().contains("absolute"));
    let args = json!({ "game": "no-such-game", "inputs": [tmp.path()], "recursive": true });
    let (err, v) = call(&c, "compile_model", args).await;
    assert!(err && v["kind"] == "usage", "{v}");
}
