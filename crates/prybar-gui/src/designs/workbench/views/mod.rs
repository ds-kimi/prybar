//! The center of the window: one view per screen.

mod decompile;
mod forms;
mod home;
mod model;
mod package;
mod preview;

use gpui::{div, prelude::*, AnyElement, Context};

use super::motion;
use crate::app::Root;
use crate::state::{AppState, Document, Screen};
use crate::theme::Palette;

/// Each screen slides in when it becomes current.
pub fn screen(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let ix = Screen::ALL.iter().position(|x| *x == s.screen).unwrap_or(0);
    let body = div()
        .id("wb-view")
        .debug_selector(|| "wb-view".into())
        .size_full()
        .p_4()
        .flex()
        .flex_col()
        .gap_3()
        .overflow_y_scroll()
        .child(content(s, p, cx));
    let wrap = div().flex_1().min_h_0().child(body);
    motion::enter(wrap, ("wb-view-in", ix))
}

fn content(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let doc = s.selected.and_then(|i| s.documents.get(i));
    match (s.screen, doc) {
        (Screen::Home, _) => home::view(s, p, cx),
        (Screen::Inspector, Some(d @ Document::Model { .. })) => model::view(d, p),
        (Screen::Package, Some(Document::Package { report, .. })) => {
            package::view(report, s.search.as_deref(), p, cx)
        }
        (Screen::Decompile, _) => decompile::view(s, p, cx),
        (Screen::Compile, _) => forms::compile(s, p, cx),
        (Screen::Publish, _) => forms::publish(s, p),
        (Screen::Settings, _) => forms::settings(s, p),
        (screen, _) => match screen.placeholder() {
            Some(ph) => preview::preview(ph, doc, p),
            None => empty(screen, p, cx),
        },
    }
}

/// A screen that needs a file of the right kind.
fn empty(screen: Screen, p: Palette, cx: &Context<Root>) -> AnyElement {
    let want = match screen {
        Screen::Package => "Open a VPK, GMA or other package to browse it.",
        _ => "Open a model to inspect it.",
    };
    let go = super::widgets::button("wb-empty-open", "Open file", true, true, p).on_click(
        cx.listener(|this, _, _, cx| this.state.update(cx, |s, cx| s.navigate(Screen::Home, cx))),
    );
    div()
        .flex()
        .flex_col()
        .items_start()
        .gap_3()
        .text_color(p.muted)
        .child(want)
        .child(go)
        .into_any_element()
}
