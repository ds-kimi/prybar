use super::s;
use crate::fixture::{json, prybar};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use tempfile::tempdir;

/// A config whose studiomdl is a shell script run through `/bin/sh` (as
/// the Wine runner would run the real one): QCs named `bad*` fail.
fn fake_game(tmp: &Path) {
    let game = tmp.join("game");
    std::fs::create_dir_all(&game).unwrap();
    let tool = tmp.join("studiomdl.sh");
    let script = "for a; do qc=$a; done\ncase $(basename \"$qc\") in\n  bad*) echo \"ERROR: broken $qc\"; exit 1;;\nesac\necho \"writing $qc.mdl\"\n";
    std::fs::write(&tool, script).unwrap();
    std::fs::set_permissions(&tool, std::fs::Permissions::from_mode(0o755)).unwrap();
    let cfg = format!(
        "[[games]]\nname = \"Test\"\ngame_dir = '{}'\nstudiomdl = '{}'\n",
        s(&game),
        s(&tool)
    );
    std::fs::write(tmp.join("config.toml"), cfg).unwrap();
}

fn qcs(dir: &Path, names: &[&str]) {
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    for n in names {
        std::fs::write(dir.join(n), "$modelname a.mdl\n").unwrap();
    }
    std::fs::write(dir.join("sub/deep.qc"), "").unwrap();
    std::fs::write(dir.join("part.qci"), "").unwrap();
}

#[test]
fn folder_compile_reports_each_qc_and_exits_partial() {
    let tmp = tempdir().unwrap();
    fake_game(tmp.path());
    let src = tmp.path().join("src");
    qcs(&src, &["a.qc", "bad.qc"]);
    let run = |extra: &[&str], code| {
        let mut cmd = prybar(tmp.path());
        cmd.env("PRYBAR_WINE", "/bin/sh");
        cmd.args(["compile", s(&src), "-g", "test", "-j", "2"])
            .args(extra);
        json(cmd, code)
    };
    let r = run(&[], 7);
    assert_eq!(
        (r["compiled"].as_u64(), r["failed"].as_u64()),
        (Some(1), Some(1))
    );
    let bad = &r["items"][1];
    assert_eq!(
        (bad["status"].as_str(), bad["code"].as_u64()),
        (Some("failed"), Some(6))
    );
    assert!(bad["errors"][0]["text"]
        .as_str()
        .unwrap()
        .contains("broken"));
    assert!(r["items"][0]["log"].is_null());
    assert_eq!(run(&["-r"], 7)["items"].as_array().unwrap().len(), 3);
}

#[test]
fn wildcard_and_missing_inputs() {
    let tmp = tempdir().unwrap();
    fake_game(tmp.path());
    let src = tmp.path().join("src");
    qcs(&src, &["a.qc", "b.qc", "bad.qc"]);
    let mut cmd = prybar(tmp.path());
    cmd.env("PRYBAR_WINE", "/bin/sh");
    cmd.args(["compile", s(&src.join("?.qc")), "-g", "test"]);
    assert_eq!(json(cmd, 0)["compiled"].as_u64(), Some(2));
    let mut cmd = prybar(tmp.path());
    cmd.env("PRYBAR_WINE", "/bin/sh");
    cmd.args(["compile", s(&src.join("*")), "-g", "test"]);
    assert_eq!(json(cmd, 7)["items"].as_array().unwrap().len(), 3);
    let mut cmd = prybar(tmp.path());
    let gone = src.join("gone");
    cmd.args(["compile", s(&gone), s(&src.join("*.nope")), "-g", "test"]);
    assert_eq!(json(cmd, 3)["failed"].as_u64(), Some(2));
}
