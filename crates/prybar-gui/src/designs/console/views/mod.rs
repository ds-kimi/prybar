//! One pane per screen; each slides in when it becomes current.

mod compile;
mod forms;
mod home;
mod model;
mod more;
mod package;

use std::path::PathBuf;

use gpui::{div, prelude::*, AnyElement, Context};

use super::{motion, widgets};
use crate::app::Root;
use crate::state::{AppState, Document, Screen};
use crate::theme::Palette;

pub fn screen(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let ix = Screen::ALL.iter().position(|x| *x == s.screen).unwrap_or(0);
    let body = div()
        .id("cs-view")
        .debug_selector(|| "cs-view".into())
        .size_full()
        .px_4()
        .py_3()
        .flex()
        .flex_col()
        .gap_3()
        .overflow_y_scroll()
        .child(content(s, p, cx));
    motion::pane(div().flex_1().min_h_0().child(body), ("cs-pane", ix))
}

fn content(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let doc = s.selected.and_then(|i| s.documents.get(i));
    match (s.screen, doc) {
        (Screen::Home, _) => home::view(s, p, cx),
        (Screen::Inspector, Some(d @ Document::Model { .. })) => model::view(d, p),
        (Screen::Package, Some(Document::Package { report, .. })) => {
            package::view(report, s.search.as_deref(), p, cx)
        }
        (Screen::Decompile, _) => forms::decompile(s, p, cx),
        (Screen::Compile, _) => compile::compile(s, p, cx),
        (Screen::Publish, _) => more::publish(s, p),
        (Screen::Settings, _) => forms::settings(s, p, cx),
        (screen, _) => match screen.placeholder() {
            Some(ph) => more::preview(ph, doc, p),
            None => empty(screen, p),
        },
    }
}

/// Output goes to a temp folder until a folder picker lands.
pub fn out_dir(kind: &str) -> PathBuf {
    std::env::temp_dir().join("prybar-gui-out").join(kind)
}

/// A screen that needs a file of the right kind says how to get one.
fn empty(screen: Screen, p: Palette) -> AnyElement {
    let what = match screen {
        Screen::Package => "no package open",
        _ => "no model open",
    };
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(widgets::heading(screen.label(), what, p))
        .child(
            div()
                .text_color(p.muted)
                .child("type a path, or `open file` in the bar"),
        )
        .into_any_element()
}
