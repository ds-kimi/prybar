use gpui::{Modifiers, TestAppContext, VisualTestContext};

use super::{forms, window};
use crate::state::tools::CompileOutcome;
use crate::state::Screen;

fn click(cx: &mut VisualTestContext, selector: &'static str) {
    let b = cx
        .debug_bounds(selector)
        .unwrap_or_else(|| panic!("{selector} drawn"));
    cx.simulate_mouse_move(b.center(), None, Modifiers::none());
    cx.simulate_click(b.center(), Modifiers::none());
    cx.run_until_parked();
}

/// A typed QC enables Compile; the fake log lands in the log view.
#[gpui::test]
fn compile_screen_runs_and_shows_the_log(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    let tmp = tempfile::tempdir().expect("tempdir");
    let qc = tmp.path().join("a.qc");
    std::fs::write(&qc, "$modelname x.mdl\n").expect("qc written");
    state.update(&mut cx, |s, cx| s.navigate(Screen::Compile, cx));
    cx.run_until_parked();
    let f = forms(&mut cx);
    f.qc.update(&mut cx, |i, cx| i.set_text(&qc.display().to_string(), cx));
    cx.run_until_parked();
    click(&mut cx, "pb-compile");
    let outcome = state.read_with(&cx, |s, _| s.tools.compile.outcome.clone());
    assert!(matches!(outcome, Some(CompileOutcome::Built { .. })));
    assert!(cx.debug_bounds("pb-log").is_some());
    click(&mut cx, "pb-log-copy");
    let copied = cx.read_from_clipboard().and_then(|c| c.text());
    assert!(copied.is_some_and(|t| t.contains("WARNING: no weights")));
}

/// Picking a game fills its folder; detect adds the found ones.
#[gpui::test]
fn settings_edit_and_detect(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, true);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Settings, cx));
    cx.run_until_parked();
    click(&mut cx, "pb-set-game-1");
    let f = forms(&mut cx);
    let dir = f.game_dir.read_with(&cx, |i, _| i.text().to_owned());
    assert!(dir.ends_with("Team Fortress 2"));
    assert!(cx.debug_bounds("pb-save-game").is_some());
    click(&mut cx, "pb-detect");
    assert_eq!(state.read_with(&cx, |s, _| s.tools.games.len()), 3);
}

/// Dry run needs a title and existing content, then reports back.
#[gpui::test]
fn publish_dry_run_from_the_form(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    let tmp = tempfile::tempdir().expect("tempdir");
    let gma = tmp.path().join("a.gma");
    std::fs::write(&gma, b"GMAD").expect("gma written");
    super::details::new_popup(&state, &mut cx);
    click(&mut cx, "pb-pub-dry");
    assert!(
        state.read_with(&cx, |s, _| s.tools.publish.outcome.is_none()),
        "no title yet"
    );
    let f = forms(&mut cx);
    f.publish
        .title
        .update(&mut cx, |i, cx| i.set_text("Crate", cx));
    f.publish
        .content
        .update(&mut cx, |i, cx| i.set_text(&gma.display().to_string(), cx));
    cx.run_until_parked();
    click(&mut cx, "pb-pub-dry");
    let ok = state.read_with(
        &cx,
        |s, _| matches!(&s.tools.publish.outcome, Some(Ok(r)) if r.dry_run),
    );
    assert!(ok);
    assert!(cx.debug_bounds("pb-pub-result").is_some());
}
