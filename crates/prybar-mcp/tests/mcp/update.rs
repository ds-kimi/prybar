use super::client::{call, connect};
use serde_json::json;

/// Offline (nothing listens on port 9): the tool answers, without `latest`.
/// The only test touching these variables, so no other test races on them.
#[tokio::test]
async fn check_update_is_offline_safe_and_never_installs() {
    let cache = tempfile::tempdir().unwrap();
    std::env::set_var("PRYBAR_UPDATE_URL", "http://127.0.0.1:9/");
    std::env::set_var("PRYBAR_UPDATE_CACHE", cache.path());
    let client = connect().await;
    let (err, v) = call(&client, "check_update", json!({})).await;
    assert!(!err, "{v}");
    assert!(v["latest"].is_null());
    assert_eq!(v["update_available"], false);
    assert_eq!(v["current"], env!("CARGO_PKG_VERSION"));
    assert!(v["command"].is_null());
    let info = client.peer_info().unwrap();
    let text = info.instructions.clone().unwrap_or_default();
    assert!(text.contains("Source engine") && !text.contains("Update:"));
}
