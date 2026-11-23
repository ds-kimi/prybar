//! The preview image exactly as the Workshop will show it: a center
//! cropped square, with the size and whether Steam gets it as is.

use std::cell::RefCell;
use std::path::PathBuf;

use gpui::{div, prelude::*, AnyElement, Context};
use prybar_workshop::preview::{describe, PreviewInfo};
use prybar_workshop::ListedItem;

use crate::app::{Forms, Root};
use crate::state::tools::Thumb;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub(super) fn line(text: String, bad: bool, t: Tokens) -> gpui::Div {
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

/// In its own column the notes go under the square; stacked, beside it.
pub fn view(
    s: &AppState,
    f: &Forms,
    editing: Option<&ListedItem>,
    wide: bool,
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
    let lines = about(&typed, editing.is_some(), t);
    let notes = div().flex_1().min_w_0().flex().flex_col().gap_1();
    div()
        .flex()
        .when(wide, |el| el.flex_col())
        .gap_3()
        .child(super::frame::frame(shown, f, wide, t, cx))
        .child(notes.children(lines))
        .into_any_element()
}

const KEEP: &str = "Steam shows a 512×512 square under 1 MB. The current image stays.";
const NONE: &str =
    "Steam shows a 512×512 square under 1 MB. No image yet: Steam shows a grey tile.";

/// Size, what Steam will get, and why it cannot be sent when it cannot;
/// with no file picked, what happens without one.
fn about(typed: &str, editing: bool, t: Tokens) -> Vec<gpui::Div> {
    if typed.is_empty() {
        return vec![line(if editing { KEEP } else { NONE }.into(), false, t)];
    }
    let path = &PathBuf::from(typed);
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
