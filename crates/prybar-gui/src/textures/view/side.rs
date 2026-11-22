//! The material list: one row per slot, found or missing at a glance.

use gpui::{div, prelude::*, AnyElement, Context};

use super::{Status, TextureView};
use crate::designs::Tipped as _;
use crate::textures::Slot;
use crate::theme::prybar::Tokens;

pub fn list(v: &TextureView, t: &Tokens, cx: &mut Context<TextureView>) -> impl IntoElement {
    let slots = v.set.as_ref().map_or(&[][..], |s| &s.slots[..]);
    let rows: Vec<_> = slots
        .iter()
        .enumerate()
        .map(|(i, s)| row(i, s, i == v.selected, t, cx))
        .collect();
    div()
        .id("pb-tex-list")
        .w_64()
        .flex_none()
        .flex()
        .flex_col()
        .gap_0p5()
        .overflow_y_scroll()
        .children(rows)
}

/// Name on top, base texture under it; missing ones say so in words and
/// color, not color alone.
fn row(i: usize, s: &Slot, on: bool, t: &Tokens, cx: &mut Context<TextureView>) -> AnyElement {
    let (hover, active) = (t.hover, t.active);
    let (found, under) = (s.info.is_some(), s.summary());
    div()
        .id(("pb-tex", i))
        .debug_selector(move || format!("pb-tex-{i}"))
        .tip("Show this texture", *t)
        .px_2()
        .py_1()
        .rounded_md()
        .cursor_pointer()
        .when(on, |el| el.bg(hover))
        .hover(move |st| st.bg(hover))
        .active(move |st| st.bg(active))
        .on_click(cx.listener(move |v, _, _, cx| v.select(i, cx)))
        .child(
            div()
                .text_sm()
                .text_color(if on { t.rust } else { t.text })
                .child(s.report.name.clone()),
        )
        .child(
            div()
                .text_xs()
                .truncate()
                .text_color(if found { t.muted } else { t.danger })
                .child(under),
        )
        .into_any_element()
}

/// Centered words while there is no image: loading, failure, or why.
pub fn message(v: &TextureView, t: &Tokens) -> Option<impl IntoElement> {
    let (text, color) = match &v.status {
        Status::Idle => ("Open a .mdl to see its textures".to_owned(), t.muted),
        Status::Loading => ("Finding textures".to_owned(), t.muted),
        Status::Failed(e) => (e.clone(), t.danger),
        Status::Ready if v.shown.is_some() => return None,
        Status::Ready if v.set.as_ref().is_some_and(|s| s.slots.is_empty()) => {
            ("This model has no materials".to_owned(), t.muted)
        }
        Status::Ready => ("No texture to show for this material".to_owned(), t.muted),
    };
    let place = div()
        .absolute()
        .size_full()
        .flex()
        .items_center()
        .justify_center();
    Some(place.text_xs().text_color(color).child(text))
}

/// A toggle with hover, press and on states.
pub fn chip(
    id: &'static str,
    label: &'static str,
    on: bool,
    t: &Tokens,
    cx: &mut Context<TextureView>,
    act: impl Fn(&mut TextureView, &mut Context<TextureView>) + 'static,
) -> impl IntoElement {
    let (hover, active) = (t.hover, t.active);
    div()
        .id(id)
        .tip(super::about::texture(id), *t)
        .debug_selector(move || id.into())
        .px_2()
        .py_0p5()
        .rounded_md()
        .text_xs()
        .border_1()
        .border_color(if on { t.rust } else { t.border })
        .text_color(if on { t.text } else { t.muted })
        .hover(move |s| s.bg(hover))
        .active(move |s| s.bg(active))
        .on_click(cx.listener(move |v, _, _, cx| {
            act(v, cx);
            cx.notify()
        }))
        .child(label)
}
