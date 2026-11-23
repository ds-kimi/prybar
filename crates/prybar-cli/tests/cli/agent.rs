use super::fixture::{json, prybar};
use super::vtf_support;

#[test]
fn texture_writes_png_and_preview_checks_it() {
    let tmp = tempfile::tempdir().unwrap();
    let vtf = tmp.path().join("wood.vtf");
    std::fs::write(
        &vtf,
        vtf_support::vtf(5, 13, (4, 4), 1, &vtf_support::dxt1_red_blue()),
    )
    .unwrap();
    let png = tmp.path().join("wood.png");
    let mut cmd = prybar(tmp.path());
    cmd.arg("texture").arg(&vtf).arg("-o").arg(&png);
    let v = json(cmd, 0);
    assert_eq!(
        (v["width"].as_u64(), v["format"].as_str()),
        (Some(4), Some("DXT1"))
    );
    let mut cmd = prybar(tmp.path());
    cmd.args(["workshop", "preview"]).arg(&png);
    let v = json(cmd, 0);
    assert_eq!(
        (v["ok"].as_bool(), v["resized"].as_bool()),
        (Some(true), Some(true))
    );
    let mut cmd = prybar(tmp.path());
    cmd.args(["workshop", "preview"])
        .arg(tmp.path().join("none.png"));
    json(cmd, 3);
}

#[test]
fn bbcode_reads_a_file_and_privacy_round_trips() {
    let tmp = tempfile::tempdir().unwrap();
    let desc = tmp.path().join("d.txt");
    std::fs::write(&desc, "[b]Big[/b] crate").unwrap();
    let mut cmd = prybar(tmp.path());
    cmd.args(["workshop", "bbcode"]).arg(&desc);
    let v = json(cmd, 0);
    assert_eq!(
        (v["plain"].as_str(), v["chars"].as_u64()),
        (Some("Big crate"), Some(16))
    );
    let mut cmd = prybar(tmp.path());
    cmd.args(["privacy", "--off"]);
    assert_eq!(json(cmd, 0)["on"], false);
    let mut cmd = prybar(tmp.path());
    cmd.arg("privacy");
    assert_eq!(json(cmd, 0)["on"], false);
}

#[test]
fn workshop_items_rejects_bad_sort_and_games() {
    let tmp = tempfile::tempdir().unwrap();
    let mut cmd = prybar(tmp.path());
    cmd.args(["workshop", "items", "garrysmod", "--sort", "loud"]);
    json(cmd, 2);
    let mut cmd = prybar(tmp.path());
    cmd.args(["workshop", "item", "l4d2", "1"]);
    json(cmd, 2);
}
