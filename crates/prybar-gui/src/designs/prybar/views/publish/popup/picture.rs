//! The preview image exactly as the Workshop will show it: a center
//! cropped square, with the size and whether Steam gets it as is.

use std::cell::RefCell;
use std::path::PathBuf;

use gpui::{div, img, prelude::*, px, AnyElement, Context, ObjectFit};
use prybar_workshop::preview::{describe, PreviewInfo};
use prybar_workshop::ListedItem;

use super::super::super::super::widgets::Tipped as _;
use crate::app::{Forms, Root};
use crate::state::tools::Thumb;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

fn line(text: String, bad: bool, t: Tokens) -> gpui::Div {
    div()
        .text_xs()
        .text_color(if bad { t.danger } else { t.muted })
        .child(text)
}

thread_local! {
    /// The last described file: the popup redraws on every hover and a
    /// header read each frame would be wasted work.
    static LAST: RefCell<Option<(PathBuf, Result<PreviewInfo, String>)>> = const { RefCell::new(None) };
}

pub fn view(
    s: &AppState,
    f: &Forms,
    editing: Option<&ListedItem>,
    t: Tokens,
    cx: &Context<Root>,
) -> AnyElement {
    let typed = f.publish.preview.read(cx).text().trim().to_owned();
    let current = editing.and_then(|it| match s.tools.steam.catalog.thumbs.get(&it.item_id) {
        Some(Thumb::Ready(p)) => Some(p.clone()),
        _ => None,
    });
    let shown = (!typed.is_empty())
        .then(|| PathBuf::from(&typed))
        .or(current);
    let lines = match typed.is_empty() {
        true => vec![line(
            if editing.is_some() { KEEP } else { NONE }.into(),
            false,
            t,
        )],
        false => about(&PathBuf::from(&typed), t),
    };
    div()
        .flex()
        .gap_3()
        .child(frame(shown, f, t, cx))
        .child(div().flex().flex_col().gap_1().children(lines))
        .into_any_element()
}

/// The square itself; a click picks another file.
fn frame(shown: Option<PathBuf>, f: &Forms, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let target = f.publish.preview.clone();
    let inner = match shown {
        Some(p) => img(p)
            .size_full()
            .object_fit(ObjectFit::Cover)
            .into_any_element(),
        None => line("Click to pick".into(), false, t)
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .into_any_element(),
    };
    div()
        .id("pb-pub-picture")
        .debug_selector(|| "pb-pub-picture".into())
        .size(px(160.))
        .flex_none()
        .rounded_md()
        .overflow_hidden()
        .border_1()
        .border_color(t.border)
        .bg(t.active)
        .cursor_pointer()
        .hover(move |s| s.border_color(t.rust))
        .tip(PICK, t)
        .on_click(cx.listener(move |this, _, _, cx| this.pick_into(target.clone(), false, cx)))
        .child(inner)
}

const PICK: &str = "Click to pick an image. The Workshop shows it as this square.";
const KEEP: &str = "Steam shows a 512×512 square under 1 MB. The current image stays.";
const NONE: &str =
    "Steam shows a 512×512 square under 1 MB. No image yet: Steam shows a grey tile.";

/// Size, what Steam will get, and why it cannot be sent when it cannot.
fn about(path: &PathBuf, t: Tokens) -> Vec<gpui::Div> {
    let info = LAST.with_borrow_mut(|last| {
        if last.as_ref().is_none_or(|(p, _)| p != path) {
            *last = Some((path.clone(), describe(path)));
        }
        last.as_ref().map(|(_, i)| i.clone())
    });
    let Some(Ok(i)) = info else {
        return vec![line("Not an image prybar can read.".into(), true, t)];
    };
    let what = if i.resized {
        "cropped to the 512×512 square and resized"
    } else {
        "sent as is"
    };
    let kb = i.bytes.div_ceil(1024);
    let mut out = vec![line(
        format!("{}×{}, {kb} KB: {what}.", i.width, i.height),
        false,
        t,
    )];
    out.extend(i.problem.map(|p| line(p, true, t)));
    out
}
