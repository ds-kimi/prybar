use super::fixture::{json, model, prybar};
use std::path::Path;
use tempfile::tempdir;

fn s(p: &Path) -> &str {
    p.to_str().unwrap()
}

#[test]
fn help_lists_every_command_and_exit_codes() {
    let out = prybar(Path::new(".")).arg("--help").assert().success();
    let text = String::from_utf8_lossy(&out.get_output().stdout).into_owned();
    let cmds = [
        "inspect",
        "decompile",
        "list",
        "unpack",
        "pack",
        "compile",
        "view",
        "publish",
        "games",
    ];
    for cmd in cmds {
        assert!(text.contains(cmd), "missing {cmd}");
    }
    assert!(text.contains("Exit codes") && text.contains("--json"));
    prybar(Path::new(".")).arg("frobnicate").assert().code(2);
}

#[test]
fn games_compile_and_view_use_the_config() {
    let tmp = tempdir().unwrap();
    let game = tmp.path().join("game");
    std::fs::create_dir_all(&game).unwrap();
    let cfg = format!("[[games]]\nname = \"Test\"\ngame_dir = '{}'\n", s(&game));
    std::fs::write(tmp.path().join("config.toml"), cfg).unwrap();
    let mut cmd = prybar(tmp.path());
    cmd.arg("games");
    assert_eq!(json(cmd, 0)["games"][0]["name"], "Test");
    let qc = tmp.path().join("a.qc");
    std::fs::write(&qc, "$modelname a.mdl\n").unwrap();
    let mut cmd = prybar(tmp.path());
    cmd.args(["compile", s(&qc), "-g", "test"]);
    let err = json(cmd, 2)["error"]["message"]
        .as_str()
        .unwrap()
        .to_owned();
    assert!(err.contains("studiomdl"), "{err}");
    prybar(tmp.path())
        .args(["compile", s(&qc), "-g", "nope"])
        .assert()
        .code(2);
    let mdl = model(tmp.path(), "a");
    prybar(tmp.path())
        .args(["view", s(&mdl), "-g", "test"])
        .assert()
        .code(2);
}

#[test]
fn publish_dry_run_validates() {
    let tmp = tempdir().unwrap();
    let content = tmp.path().join("content");
    std::fs::create_dir_all(&content).unwrap();
    let spec = serde_json::json!({
        "app_id": 440, "title": "Crate", "content": content, "tags": ["props"]
    });
    let path = tmp.path().join("item.json");
    std::fs::write(&path, spec.to_string()).unwrap();
    let mut cmd = prybar(tmp.path());
    cmd.args(["publish", s(&path), "--dry-run"]);
    let r = json(cmd, 0);
    assert_eq!(
        (r["dry_run"].as_bool(), r["created"].as_bool()),
        (Some(true), Some(true))
    );
    let bad = serde_json::json!({ "app_id": 440, "title": "a\u{0}b" });
    std::fs::write(&path, bad.to_string()).unwrap();
    prybar(tmp.path())
        .args(["publish", s(&path), "--dry-run"])
        .assert()
        .code(4);
    prybar(tmp.path()).arg("publish").assert().code(2);
}
