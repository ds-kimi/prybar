//! The body under the step path: one view per screen.

mod decompile;
mod forms;
mod home;
mod model;
mod package;
mod preview;
mod publish;
mod result;

use gpui::{div, prelude::*, AnyElement, Context};

use super::{motion, widgets};
use crate::app::Root;
use crate::state::{AppState, Document, Screen};
use crate::theme::Palette;

/// Each screen rises in when it becomes current.
pub fn screen(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let ix = Screen::ALL.iter().position(|x| *x == s.screen).unwrap_or(0);
    let body = div()
        .id("cv-view")
        .debug_selector(|| "cv-view".into())
        .size_full()
        .flex()
        .flex_col()
        .gap_4()
        .overflow_y_scroll()
        .child(content(s, p, cx));
    motion::enter(div().flex_1().min_h_0().child(body), ("cv-view-in", ix), 0)
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
        (Screen::Publish, _) => publish::publish(s, p),
        (Screen::Settings, _) => forms::settings(s, p, cx),
        (screen, _) => match screen.placeholder() {
            Some(ph) => preview::view(ph, doc, p),
            None => empty(screen, p, cx),
        },
    }
}

/// Output goes to a temp folder until a folder picker lands.
fn out_dir() -> std::path::PathBuf {
    std::env::temp_dir()
        .join("prybar-gui-out")
        .join("decompile")
}

/// A screen that needs a file of the right kind points back to the drop zone.
fn empty(screen: Screen, p: Palette, cx: &Context<Root>) -> AnyElement {
    let (head, hint) = match screen {
        Screen::Package => ("No package yet", "Drop a VPK or GMA to look inside."),
        _ => ("No model yet", "Drop a .mdl to see what it holds."),
    };
    let go = widgets::pill("cv-empty-open", "Go to drop zone", true, true, p).on_click(
        cx.listener(|this, _, _, cx| this.state.update(cx, |s, cx| s.navigate(Screen::Home, cx))),
    );
    div()
        .py_16()
        .flex()
        .flex_col()
        .items_center()
        .gap_4()
        .child(div().size_16().rounded_full().bg(p.hover))
        .child(div().text_xl().child(head))
        .child(div().text_color(p.muted).child(hint))
        .child(go)
        .into_any_element()
}
