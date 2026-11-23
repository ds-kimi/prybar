use std::sync::Arc;

use super::{click, description, forms};
use crate::designs::bbcode::Shot;

/// Every tag Steam lists, in one description.
const ALL: &str = "[h1]Big[/h1][h2]Mid[/h2][h3]Small[/h3]\n[b]b[/b] [i]i[/i] [u]u[/u] \
    [strike]s[/strike] [noparse][b]raw[/b][/noparse] [url=https://example.com]link[/url] \
    [url]https://example.com[/url]\n[spoiler]secret[/spoiler][hr]\
    [list][*]one[*]two[/list][olist][*]first[/olist][quote=Ann]said[/quote][quote]anon[/quote]\
    [code]let x = 1;\n  y[/code][table noborder=1][tr][th]H[/th][/tr][tr][td]c[/td][/tr][/table]\
    [url=https://example.com][img]https://example.com/badge.png[/img][/url][img]http://x[/img]\
    [color]unknown[/color] [b]open";

/// Preview draws every tag, fetches images through the editor's
/// fetcher and goes back to the field.
#[gpui::test]
fn preview_draws_every_tag(cx: &mut gpui::TestAppContext) {
    let (mut cx, _desc) = description(cx, ALL);
    let editor = forms(&mut cx).publish.editor;
    let png = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/prybar.png");
    let file = png.clone();
    let fetch = Arc::new(move |url: &str| match url.starts_with("https://") {
        true => Ok(file.clone()),
        false => Err("not https".to_owned()),
    });
    editor.update(&mut cx, |e, _| e.set_fetch(fetch));
    click(&mut cx, "pb-bb-preview");
    for id in ["pb-bb-preview-body", "pb-bb-spoiler", "pb-bb-img"] {
        assert!(cx.debug_bounds(id).is_some(), "{id} drawn");
    }
    assert!(cx.debug_bounds("pb-bb-toolbar").is_some());
    assert!(
        cx.debug_bounds("pb-bb-b").is_none(),
        "no format buttons in preview"
    );
    let shots = editor.read_with(&cx, |e, _| e.shots().clone());
    assert_eq!(
        shots.get("https://example.com/badge.png"),
        Some(&Shot::Ready(png))
    );
    assert_eq!(shots.get("http://x"), Some(&Shot::Failed));
    click(&mut cx, "pb-bb-edit");
    assert!(cx.debug_bounds("pb-bb-preview-body").is_none());
    assert!(cx.debug_bounds("pb-bb-b").is_some());
}

/// The popup's close button is a square inset like the header text.
#[gpui::test]
fn close_button_is_inset(cx: &mut gpui::TestAppContext) {
    let (mut cx, _desc) = description(cx, "");
    let card = cx.debug_bounds("pb-modal").expect("card");
    let close = cx.debug_bounds("pb-modal-close").expect("close");
    assert_eq!(close.size.width, close.size.height, "square");
    let right = f32::from(card.right() - close.right());
    let top = f32::from(close.top() - card.top());
    assert!(right >= 12. && top >= 8., "inset: right {right}, top {top}");
}
