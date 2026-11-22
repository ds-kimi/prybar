//! Pill buttons: filled orange for the main action, outlined otherwise.

use gpui::{div, prelude::*, px, Div, ElementId, SharedString, Stateful};

use crate::theme::Palette;

/// A rounded button. Disabled ones keep their place but take no input,
/// so the caller only attaches `on_click` when `enabled`.
pub fn pill(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    primary: bool,
    enabled: bool,
    p: Palette,
) -> Stateful<Div> {
    let (bg, fg) = if primary {
        (p.accent, p.surface)
    } else {
        (p.surface, p.text)
    };
    let el = div()
        .id(id)
        .h_10()
        .px_5()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .border_2()
        .border_color(if primary { p.accent } else { p.border })
        .bg(bg)
        .text_color(fg)
        .child(label.into());
    if !enabled {
        return el.opacity(0.4);
    }
    live(el, primary, p)
}

/// Hover grows the pill a little and lifts it; press squashes it back.
fn live(el: Stateful<Div>, primary: bool, p: Palette) -> Stateful<Div> {
    el.cursor_pointer()
        .tab_index(0)
        .hover(move |s| {
            let s = s.shadow_md().px(px(24.)).mt(px(-2.)).mb(px(2.));
            if primary {
                s.opacity(0.92)
            } else {
                s.border_color(p.accent)
            }
        })
        .active(move |s| s.shadow_none().px(px(18.)).mt(px(0.)).mb(px(0.)))
        .focus_visible(move |s| s.border_color(p.text))
}
