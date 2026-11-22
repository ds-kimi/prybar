//! One tab per open file, with an underline that slides to the selected
//! one; the sidebar's file list mirrors them.

use gpui::{div, prelude::*, px, AnyElement, Context};

use super::super::{motion, widgets};
use crate::actions::Open;
use crate::app::Root;
use crate::state::AppState;
use crate::theme::Palette;

/// Fixed tab width, so the underline position is a plain multiple.
const TAB: f32 = 168.;

pub fn bar(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let tabs = s
        .documents
        .iter()
        .enumerate()
        .map(|(ix, doc)| tab(ix, doc.title(), s.selected == Some(ix), p, cx));
    let underline = s.selected.map(|ix| underline(ix, p));
    let empty = s.documents.is_empty().then(|| {
        div()
            .px_3()
            .flex()
            .items_center()
            .text_color(p.muted)
            .child("No file open")
    });
    div()
        .h_9()
        .flex_none()
        .relative()
        .flex()
        .bg(p.surface)
        .border_b_1()
        .border_color(p.border)
        .children(tabs)
        .children(underline)
        .children(empty)
        .into_any_element()
}

fn tab(
    ix: usize,
    title: String,
    current: bool,
    p: Palette,
    cx: &Context<Root>,
) -> impl IntoElement {
    div()
        .id(("wb-tab", ix))
        .debug_selector(move || format!("tab-{ix}"))
        .w(px(TAB))
        .h_full()
        .px_3()
        .flex()
        .items_center()
        .overflow_hidden()
        .border_r_1()
        .border_color(p.border)
        .cursor_pointer()
        .text_color(if current { p.text } else { p.muted })
        .when(current, |el| el.bg(p.bg))
        .hover(move |st| st.bg(p.hover).text_color(p.text))
        .active(move |st| st.bg(p.active))
        .on_click(
            cx.listener(move |this, _, _, cx| this.state.update(cx, |s, cx| s.select(ix, cx))),
        )
        .child(title)
}

fn underline(ix: usize, p: Palette) -> AnyElement {
    let line = div()
        .absolute()
        .bottom_0()
        .w(px(TAB))
        .h(px(2.))
        .bg(p.accent);
    motion::slide_to(line, "wb-tab-line", ix as f32 * TAB, true)
}

pub fn file_list(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let rows = s.documents.iter().enumerate().map(|(ix, doc)| {
        widgets::row(("wb-file", ix), s.selected == Some(ix), p)
            .on_click(
                cx.listener(move |this, _, _, cx| this.state.update(cx, |s, cx| s.select(ix, cx))),
            )
            .child(
                div()
                    .size_1p5()
                    .rounded_full()
                    .bg(if s.selected == Some(ix) {
                        p.accent
                    } else {
                        p.border
                    }),
            )
            .child(doc.title())
    });
    let open = widgets::button("wb-open", "Open file", false, true, p)
        .mt_2()
        .on_click(|_, window, cx| window.dispatch_action(Box::new(Open), cx));
    div()
        .flex()
        .flex_col()
        .children(rows)
        .child(open)
        .into_any_element()
}
