//! The few pieces the console is built from: rows, key hints, headings
//! and bracketed actions.

use gpui::{div, prelude::*, px, Div, ElementId, SharedString, Stateful};

use crate::theme::Palette;

/// Height of every list row; the highlight spring moves in these steps.
pub const ROW: f32 = 26.;

/// A full-width row that reacts to hover, press and keyboard focus.
pub fn row(id: impl Into<ElementId>, p: Palette) -> Stateful<Div> {
    div()
        .id(id)
        .h(px(ROW))
        .px_3()
        .flex()
        .items_center()
        .gap_3()
        .cursor_pointer()
        .tab_index(0)
        .border_l_2()
        .border_color(gpui::transparent_black())
        .hover(move |s| s.bg(p.hover))
        .active(move |s| s.bg(p.active))
        .focus_visible(move |s| s.border_color(p.accent))
}

/// A key cap, e.g. `⏎` or `cmd-k`.
pub fn key(k: impl Into<SharedString>, p: Palette) -> Div {
    div()
        .px_1()
        .rounded_sm()
        .border_1()
        .border_color(p.border)
        .text_xs()
        .text_color(p.muted)
        .child(k.into())
}

/// `## title  hint`, the only heading style.
pub fn heading(title: &str, hint: impl Into<SharedString>, p: Palette) -> Div {
    div()
        .flex()
        .gap_3()
        .child(div().text_color(p.accent).child(format!("## {title}")))
        .child(div().text_color(p.muted).child(hint.into()))
}

/// `[ label ]` plus the command that reaches it; muted and inert when off.
pub fn action(id: &'static str, label: &str, cmd: &str, on: bool, p: Palette) -> Stateful<Div> {
    let el = div()
        .id(id)
        .debug_selector(move || id.into())
        .flex()
        .gap_2()
        .items_center()
        .px_2()
        .border_1()
        .border_color(p.border)
        .child(format!("[ {label} ]"))
        .child(key(cmd.to_owned(), p));
    if !on {
        return el.text_color(p.muted).opacity(0.5);
    }
    el.cursor_pointer()
        .tab_index(0)
        .text_color(p.accent)
        .hover(move |s| s.bg(p.hover).border_color(p.accent))
        .active(move |s| s.bg(p.active))
        .focus_visible(move |s| s.border_color(p.accent))
}
