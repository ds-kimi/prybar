//! Images and links around blocks.

use gpui::{div, img, prelude::*, px, AnyElement, ObjectFit};
use prybar_workshop::bbcode::Node;

use super::super::super::tip::Tipped as _;
use super::super::Shot;
use super::{blocks, Look};

/// The image once fetched, fit to the preview's width; a quiet line
/// while it loads or when it cannot be shown.
pub fn image(url: &str, l: &Look) -> AnyElement {
    let t = l.t;
    let url = url.trim();
    let note = |s: String| {
        div()
            .text_xs()
            .text_color(t.muted)
            .child(s)
            .into_any_element()
    };
    match l.shots.get(url) {
        Some(Shot::Ready(p)) => div()
            .debug_selector(|| "pb-bb-img".into())
            .max_w_full()
            .child(
                img(p.clone())
                    .max_w_full()
                    .max_h(px(360.))
                    .object_fit(ObjectFit::ScaleDown),
            )
            .into_any_element(),
        Some(Shot::Failed) => note(format!("Image not shown: {url}")),
        _ => note("Loading image…".into()),
    }
}

/// A clickable block: opens the address in the browser.
pub fn link(href: String, kids: &[Node], l: &Look) -> AnyElement {
    let href = href.trim().to_owned();
    let tip = href.clone();
    div()
        .id(l.id("pb-bb-link"))
        .cursor_pointer()
        .tip(tip, l.t)
        .on_click(move |_, _, cx| {
            if href.starts_with("http") {
                cx.open_url(&href)
            }
        })
        .child(blocks(kids, l))
        .into_any_element()
}
