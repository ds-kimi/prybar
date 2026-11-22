use super::client::{call, connect, Client};
use serde_json::json;

/// One test for every config write: the tests share one config file and
/// run in parallel, so splitting these would race.
#[tokio::test]
async fn config_set_get_remove() {
    let tmp = tempfile::tempdir().unwrap();
    let c = connect().await;
    seed_tool_paths(&c, tmp.path()).await;
    let game = json!({ "name": "TestGame", "game_dir": tmp.path(), "packer": "gma" });
    let (err, v) = call(&c, "config_set", game).await;
    assert!(!err, "{v}");
    let (_, v) = call(&c, "config_get", json!({})).await;
    let names: Vec<_> = v["games"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| g["name"].clone())
        .collect();
    assert!(names.contains(&json!("TestGame")), "{v}");
    assert_eq!(v["games"][0]["studiomdl"], "/kept/studiomdl.exe", "{v}");
    let remove = json!({ "name": "testgame", "remove": true });
    let (err, _) = call(&c, "config_set", remove.clone()).await;
    assert!(!err);
    let (err, v) = call(&c, "config_set", remove).await;
    assert!(err && v["kind"] == "not_found", "{v}");
}

/// Stands in for a detected game: the only way tool paths get in.
async fn seed_tool_paths(c: &Client, game_dir: &std::path::Path) {
    let (_, v) = call(c, "config_get", json!({})).await;
    let path = v["config_path"].as_str().unwrap();
    let toml = format!(
        "[[games]]
name = \"TestGame\"
game_dir = {:?}
studiomdl = \"/kept/studiomdl.exe\"
",
        game_dir.to_str().unwrap()
    );
    std::fs::write(path, toml).unwrap();
}

#[tokio::test]
async fn games_list_does_not_save_by_default() {
    let c = connect().await;
    let (err, v) = call(&c, "games_list", json!({})).await;
    assert!(!err && v["saved"] == false, "{v}");
}

#[tokio::test]
async fn config_set_rejects_bad_fields() {
    let tmp = tempfile::tempdir().unwrap();
    let c = connect().await;
    let cases = [
        json!({ "name": "G" }),
        json!({ "name": "G", "game_dir": "rel" }),
        json!({ "name": "G", "game_dir": tmp.path(), "packer": "zip" }),
        json!({ "name": " ", "game_dir": tmp.path() }),
    ];
    for args in cases {
        let (err, v) = call(&c, "config_set", args.clone()).await;
        assert!(err && v["kind"] == "usage", "{args} -> {v}");
    }
    let tool = json!({ "name": "G", "game_dir": tmp.path(), "studiomdl": "C:/x/studiomdl.exe" });
    let (err, v) = call(&c, "config_set", tool).await;
    assert!(err && v["kind"] == "schema", "{v}");
}
