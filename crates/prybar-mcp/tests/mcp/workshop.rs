use super::client::{call, connect};
use serde_json::json;

#[tokio::test]
async fn check_bbcode_flattens_and_counts() {
    let c = connect().await;
    let (err, v) = call(
        &c,
        "check_bbcode",
        json!({ "text": "[h1]Crate[/h1][list][*]wood[/list]" }),
    )
    .await;
    assert!(!err, "{v}");
    assert_eq!(v["max_chars"], 7999);
    assert_eq!(v["tags"], json!(["h1", "list", "*"]));
    assert!(v["plain"].as_str().unwrap().contains("- wood"));
}

#[tokio::test]
async fn path_tools_refuse_relative_paths_and_missing_files() {
    let c = connect().await;
    let (err, v) = call(&c, "check_preview", json!({ "image": "a.png" })).await;
    assert!(err && v["code"] == 2, "{v}");
    let missing = std::env::temp_dir().join("prybar-no-such-preview.png");
    let (err, v) = call(&c, "check_preview", json!({ "image": missing })).await;
    assert!(err && v["code"] == 3, "{v}");
    let args = json!({ "input": "x.vtf", "out": "/tmp/x.png" });
    let (err, v) = call(&c, "decode_texture", args).await;
    assert!(err && v["code"] == 2, "{v}");
}

#[tokio::test]
async fn privacy_reads_freely_but_writes_need_confirm() {
    let c = connect().await;
    let (err, v) = call(&c, "privacy", json!({})).await;
    assert!(!err, "{v}");
    assert!(v["log"].is_array());
    let (err, v) = call(&c, "privacy", json!({ "set": false })).await;
    assert!(err && v["code"] == 2, "{v}");
    let (err, v) = call(&c, "privacy", json!({ "set": false, "confirm": true })).await;
    assert!(!err && v["on"] == false, "{v}");
}

#[tokio::test]
async fn workshop_tools_resolve_game_names() {
    let c = connect().await;
    let (err, v) = call(&c, "workshop_items", json!({ "game": "l4d2" })).await;
    assert!(err && v["code"] == 2, "{v}");
    let (err, v) = call(
        &c,
        "workshop_items",
        json!({ "game": "garrysmod", "sort": "loud" }),
    )
    .await;
    assert!(err && v["code"] == 2, "{v}");
    let (err, v) = call(&c, "workshop_status", json!({ "game": "tf2" })).await;
    assert!(!err, "{v}");
    assert!(v["state"].is_string());
}
