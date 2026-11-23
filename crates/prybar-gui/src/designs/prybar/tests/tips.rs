//! Tooltips: they show after the pause, go away with the mouse, and
//! every clickable element in the app has one.

use std::path::Path;

use gpui::{Modifiers, TestAppContext};

use super::window;
use crate::designs::prybar::widgets::tip::DELAY;

/// Clicks that need no explanation: backdrops that close a layer and
/// handlers that only stop a click from going through.
const SILENT: [(&str, usize); 1] = [("designs/prybar/shell/palette.rs", 2)];

#[gpui::test]
fn tip_shows_after_the_pause_and_hides(cx: &mut TestAppContext) {
    let (_state, mut cx) = window(cx, false);
    let b = cx.debug_bounds("nav-package").expect("nav drawn");
    cx.simulate_mouse_move(b.center(), None, Modifiers::none());
    cx.run_until_parked();
    assert!(
        cx.debug_bounds("pb-tip").is_none(),
        "no tip before the pause"
    );
    cx.executor().advance_clock(DELAY);
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-tip").is_some(), "tip after the pause");
    cx.simulate_mouse_move(
        b.center() + gpui::point(gpui::px(0.), gpui::px(300.)),
        None,
        Modifiers::none(),
    );
    cx.run_until_parked();
    assert!(
        cx.debug_bounds("pb-tip").is_none(),
        "tip gone with the mouse"
    );
}

/// Counts clicks and tips per source file; a new click without a tip
/// fails here with the file named.
#[test]
fn every_clickable_has_a_tip() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut missing = Vec::new();
    for dir in ["designs/prybar", "viewport/view", "textures/view"] {
        walk(&src, &src.join(dir), &mut missing);
    }
    assert!(missing.is_empty(), "clicks without a tooltip: {missing:#?}");
}

fn walk(src: &Path, dir: &Path, missing: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for path in entries.flatten().map(|e| e.path()) {
        let name = path
            .strip_prefix(src)
            .unwrap_or(&path)
            .display()
            .to_string()
            // Windows paths use `\`; the SILENT list uses `/`.
            .replace(std::path::MAIN_SEPARATOR, "/");
        if path.is_dir() && !name.ends_with("tests") {
            walk(src, &path, missing);
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let code = text.split("#[cfg(test)]").next().unwrap_or_default();
        let clicks = code.matches(".on_click(").count();
        let tips = code.matches(".tip(").count() + code.matches(".tooltip(").count();
        let silent = SILENT.iter().find(|s| s.0 == name).map_or(0, |s| s.1);
        if clicks > tips + silent {
            missing.push(format!("{name}: {clicks} clicks, {tips} tips"));
        }
    }
}
