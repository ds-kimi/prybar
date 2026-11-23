use super::{click, description, text};
use crate::actions::MOD;

/// A button wraps the selection and keeps it selected; with nothing
/// selected it inserts the tag at the caret.
#[gpui::test]
fn toolbar_wraps_and_inserts(cx: &mut gpui::TestAppContext) {
    let (mut cx, desc) = description(cx, "hello");
    cx.simulate_keystrokes(&format!("{MOD}-a"));
    click(&mut cx, "pb-bb-b");
    assert_eq!(text(&mut cx, &desc), "[b]hello[/b]");
    assert_eq!(desc.read_with(&cx, |i, _| i.selected().to_owned()), "hello");
    cx.simulate_keystrokes("right");
    click(&mut cx, "pb-bb-hr");
    assert_eq!(text(&mut cx, &desc), "[b]hello[hr][/b]");
    click(&mut cx, "pb-bb-list");
    assert!(text(&mut cx, &desc).contains("[list]\n[*]\n[/list]"));
    cx.simulate_keystrokes(&format!("{MOD}-z"));
    assert_eq!(
        text(&mut cx, &desc),
        "[b]hello[hr][/b]",
        "one undo per button"
    );
    click(&mut cx, "pb-bb-olist");
    cx.simulate_input("one");
    assert!(text(&mut cx, &desc).contains("[olist]\n[*]one\n[/olist]"));
}

/// Mod-b, i and u wrap; mod-k asks for the address, then links.
#[gpui::test]
fn shortcuts_format_and_link(cx: &mut gpui::TestAppContext) {
    let (mut cx, desc) = description(cx, "x");
    cx.simulate_keystrokes(&format!("{MOD}-a {MOD}-i"));
    assert_eq!(text(&mut cx, &desc), "[i]x[/i]");
    cx.simulate_keystrokes(&format!("{MOD}-u {MOD}-b"));
    assert_eq!(text(&mut cx, &desc), "[i][u][b]x[/b][/u][/i]");
    cx.simulate_keystrokes(&format!("{MOD}-k"));
    cx.run_until_parked();
    assert!(
        cx.debug_bounds("pb-bb-prompt").is_some(),
        "link prompt open"
    );
    cx.simulate_input("https://example.com");
    cx.simulate_keystrokes("enter");
    cx.run_until_parked();
    let got = text(&mut cx, &desc);
    assert_eq!(got, "[i][u][b][url=https://example.com]x[/url][/b][/u][/i]");
    assert!(cx.debug_bounds("pb-bb-prompt").is_none(), "prompt closed");
}

/// Long text wraps on screen without a newline in it; the field grows
/// with it, and up goes one drawn row, not to the start.
#[gpui::test]
fn soft_wrap_keeps_text(cx: &mut gpui::TestAppContext) {
    let (mut cx, desc) = description(cx, "short");
    let small = cx.debug_bounds("pb-pub-desc").expect("field").size.height;
    let long = "word ".repeat(80);
    desc.update(&mut cx, |i, cx| i.set_text(long.trim_end(), cx));
    cx.run_until_parked();
    let tall = cx.debug_bounds("pb-pub-desc").expect("field").size.height;
    assert!(
        tall > small,
        "grows with wrapped rows: {small:?} -> {tall:?}"
    );
    assert!(!text(&mut cx, &desc).contains('\n'));
    cx.simulate_keystrokes("shift-up");
    let picked = desc.read_with(&cx, |i, _| i.selected().len());
    assert!(
        picked > 0 && picked < long.len() / 2,
        "one row up: {picked}"
    );
    cx.simulate_keystrokes("enter");
    assert_eq!(
        text(&mut cx, &desc).matches('\n').count(),
        1,
        "enter adds a line"
    );
}
