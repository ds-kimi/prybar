//! The decompile switches, shared by single models and folders.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::super::widgets::{self, Tipped as _};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

type Flip = fn(&mut AppState);
type Get = fn(&AppState) -> bool;

/// Id, label, tooltip, whether it is on, and what a click changes.
const ROWS: [(&str, &str, &str, Get, Flip); 4] = [
    (
        "pb-recursive",
        "Include subfolders",
        "Also decompile the models in folders inside the input folder",
        |s| !s.batch.shallow,
        |s| s.batch.shallow = !s.batch.shallow,
    ),
    (
        "pb-tree",
        "Keep the folder tree",
        "Mirror the input folders in the output so same-named models never collide; off puts each model in <output>/<model name>/",
        |s| !s.batch.flat,
        |s| s.batch.flat = !s.batch.flat,
    ),
    (
        "pb-includes",
        "Include animation models",
        "Also decompile the models named by $includemodel (animations, gestures, postures), once each",
        |s| !s.batch.skip_includes,
        |s| s.batch.skip_includes = !s.batch.skip_includes,
    ),
    (
        "pb-split",
        "One .qci file per QC section",
        "Write each QC section to its own .qci file, included from the main QC",
        |s| s.split_qc,
        |s| s.split_qc = !s.split_qc,
    ),
];

pub fn view(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let switches = ROWS
        .into_iter()
        .map(|(id, label, tip, on, flip)| toggle(id, label, on(s), tip, flip, t, cx));
    div()
        .flex()
        .flex_col()
        .children(switches)
        .into_any_element()
}

fn toggle(
    id: &'static str,
    label: &'static str,
    on: bool,
    tip: &'static str,
    flip: Flip,
    t: Tokens,
    cx: &Context<Root>,
) -> AnyElement {
    widgets::switch(id, label, on, t)
        .debug_selector(move || id.into())
        .tip(tip, t)
        .on_click(cx.listener(move |this, _, _, cx| {
            this.state.update(cx, |s, cx| {
                flip(s);
                cx.notify();
            })
        }))
        .into_any_element()
}
