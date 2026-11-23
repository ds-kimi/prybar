//! The helpers the CLI and MCP share with the GUI: texture export,
//! description and preview checks, and the analytics choice.

#[path = "../../prybar-vtf/tests/support/mod.rs"]
mod vtf_support;

use prybar_service::workshop::{check_bbcode, check_preview};

#[test]
fn vtf_to_png_then_preview_check() {
    let tmp = tempfile::tempdir().unwrap();
    let vtf = tmp.path().join("wood.vtf");
    let bytes = vtf_support::vtf(5, 13, (4, 4), 1, &vtf_support::dxt1_red_blue());
    std::fs::write(&vtf, bytes).unwrap();
    let png = tmp.path().join("out/wood.png");
    let r = prybar_service::texture::export_file(&vtf, 9, &png).unwrap();
    assert_eq!((r.width, r.height, r.mips), (4, 4, 1));
    let p = check_preview(&png).unwrap();
    assert_eq!((p.width, p.height), (4, 4));
    assert!(p.ok && p.resized);
    assert!(check_preview(&tmp.path().join("none.png")).is_err());
}

#[test]
fn bad_vtf_is_invalid_input_not_a_panic() {
    let tmp = tempfile::tempdir().unwrap();
    let vtf = tmp.path().join("bad.vtf");
    std::fs::write(&vtf, b"VTF\0junk").unwrap();
    let e = prybar_service::texture::export_file(&vtf, 0, &tmp.path().join("x.png")).unwrap_err();
    assert_eq!(e.exit_code() as u8, 4);
}

#[test]
fn bbcode_report_counts_and_flattens() {
    let r = check_bbcode("[b]Hi[/b] [url=https://a.b]x[/url]");
    assert_eq!(r.tags, ["b", "url"]);
    assert_eq!(r.plain, "Hi x (https://a.b)");
    assert!(!r.too_long);
    assert!(check_bbcode(&"a".repeat(8000)).too_long);
}

/// One test touches `PRYBAR_CONFIG` so no other test races on it.
#[test]
fn privacy_choice_keeps_other_prefs() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("gui.txt"), "theme=light\ntelemetry=on\n").unwrap();
    std::env::set_var("PRYBAR_CONFIG", tmp.path().join("config.toml"));
    let r = prybar_service::privacy::privacy(Some(false), true).unwrap();
    assert!(!r.on);
    let text = std::fs::read_to_string(tmp.path().join("gui.txt")).unwrap();
    assert!(text.contains("theme=light") && text.contains("telemetry=off"));
    assert!(
        prybar_service::privacy::privacy(Some(true), false)
            .unwrap()
            .on
    );
}
