//! Workshop game targets through `prybar publish`.

use super::fixture::{json, prybar};
use tempfile::tempdir;

#[test]
fn publish_games_lists_targets_and_refuses_unsupported() {
    let tmp = tempdir().unwrap();
    let mut cmd = prybar(tmp.path());
    cmd.args(["publish", "--games"]);
    let r = json(cmd, 0);
    assert_eq!(r["supported"][1]["id"], "tf2");
    prybar(tmp.path())
        .args(["publish", "--status", "portal2"])
        .assert()
        .code(2);
}
