//! Canvas building blocks: soft cards that lift on hover, pill buttons,
//! chips and headings. Every interactive one reacts to hover, press and
//! keyboard focus; disabled ones fade and take no input.

mod button;

pub use button::pill;

use gpui::{div, prelude::*, px, Div, ElementId, SharedString, Stateful};

use crate::theme::Palette;

/// A white rounded card. Hover lifts it 3 px with a deeper shadow and an
/// accent edge; pressing pushes it back down.
pub fn card(id: impl Into<ElementId>, p: Palette) -> Stateful<Div> {
    div()
        .id(id)
        .p_4()
        .rounded_2xl()
        .border_1()
        .border_color(p.border)
        .bg(p.surface)
        .shadow_sm()
        .cursor_pointer()
        .tab_index(0)
        .hover(move |s| s.shadow_lg().border_color(p.accent).mt(px(-3.)).mb(px(3.)))
        .active(move |s| s.shadow_sm().bg(p.hover).mt(px(0.)).mb(px(0.)))
        .focus_visible(move |s| s.border_2().border_color(p.accent))
}

/// A small rounded label, tinted when `on`.
pub fn chip(text: impl Into<SharedString>, on: bool, p: Palette) -> Div {
    div()
        .px_3()
        .py_1()
        .rounded_full()
        .text_xs()
        .bg(if on { p.accent } else { p.hover })
        .text_color(if on { p.surface } else { p.muted })
        .child(text.into())
}

/// A screen heading with a friendly one-line hint under it.
pub fn title(text: &'static str, hint: impl Into<SharedString>, p: Palette) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_2xl().child(text))
        .child(div().text_sm().text_color(p.muted).child(hint.into()))
}

/// A round search bubble; accented while the search is open.
pub fn search(id: &'static str, text: String, open: bool, p: Palette) -> Stateful<Div> {
    div()
        .id(id)
        .debug_selector(move || id.into())
        .h_12()
        .px_5()
        .flex()
        .items_center()
        .rounded_full()
        .border_2()
        .border_color(if open { p.accent } else { p.border })
        .bg(p.surface)
        .text_color(if open { p.text } else { p.muted })
        .cursor_text()
        .tab_index(0)
        .hover(move |st| st.border_color(p.accent).shadow_md())
        .focus_visible(move |st| st.border_color(p.accent))
        .child(text)
}
