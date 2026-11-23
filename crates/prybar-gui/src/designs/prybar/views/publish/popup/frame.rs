//! The clickable square the preview image is shown in.

use std::path::PathBuf;

use gpui::{div, img, prelude::*, px, AnyElement, Context, ObjectFit};

use super::super::super::super::widgets::Tipped as _;
use super::picture::line;
use crate::app::{Forms, Root};
use crate::theme::prybar::Tokens;

const PICK: &str = "Click to pick an image. The Workshop shows it as this square.";

/// The square itself; a click picks another file.
pub fn frame(
    shown: Option<PathBuf>,
    f: &Forms,
    wide: bool,
    t: Tokens,
    cx: &Context<Root>,
) -> impl IntoElement {
    let target = f.publish.preview.clone();
    div()
        .id("pb-pub-picture")
        .debug_selector(|| "pb-pub-picture".into())
        .size(px(if wide { 240. } else { 160. }))
        .flex_none()
        .flex()
        .rounded_md()
        .overflow_hidden()
        .border_1()
        .border_color(t.border)
        .bg(t.active)
        .cursor_pointer()
        .hover(move |s| s.border_color(t.rust))
        .tip(PICK, t)
        .on_click(cx.listener(move |this, _, _, cx| this.pick_into(target.clone(), false, cx)))
        .child(inner(shown, t))
}

/// Cover keeps the image's aspect ratio, cropped the way Steam crops it.
fn inner(shown: Option<PathBuf>, t: Tokens) -> AnyElement {
    match shown {
        Some(p) => img(p)
            .size_full()
            .object_fit(ObjectFit::Cover)
            .into_any_element(),
        None => line("Click to pick".into(), false, t)
            .m_auto()
            .into_any_element(),
    }
}
