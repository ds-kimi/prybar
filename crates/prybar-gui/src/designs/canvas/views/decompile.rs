//! Decompile: one big card with the file, the options and a run button;
//! the result card is in `result`.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::widgets;
use super::out_dir;
use super::result;
use crate::app::Root;
use crate::state::{AppState, JobStatus};
use crate::theme::Palette;

pub fn view(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let target = s
        .selected
        .and_then(|i| s.documents.get(i))
        .map(|d| d.title());
    let hint = target
        .clone()
        .unwrap_or_else(|| "Drop a model first, then come back.".into());
    let out = format!("Saved to {}", out_dir().display());
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(widgets::title("Decompile", hint, p))
        .child(div().text_sm().text_color(p.muted).child(out))
        .child(div().flex().child(split(s.split_qc, p, cx)))
        .child(div().flex().child(run(s, target.is_some(), p, cx)))
        .child(result::view(s, p))
        .into_any_element()
}

/// The split option as a chip that fills when on.
fn split(on: bool, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    widgets::chip("One .qci per QC section", on, p)
        .id("cv-split")
        .debug_selector(|| "cv-split".into())
        .cursor_pointer()
        .tab_index(0)
        .hover(move |st| st.shadow_md())
        .active(move |st| st.opacity(0.8))
        .on_click(cx.listener(|this, _, _, cx| {
            this.state.update(cx, |s, cx| {
                s.split_qc = !s.split_qc;
                cx.notify();
            })
        }))
}

/// Disabled with no file, and while a decompile is already running.
fn run(s: &AppState, has_target: bool, p: Palette, cx: &Context<Root>) -> AnyElement {
    let busy = s
        .jobs
        .iter()
        .any(|j| j.label == "Decompile" && j.status == JobStatus::Running);
    let ready = has_target && !busy;
    let label = if busy {
        "Working on it…"
    } else {
        "Decompile it"
    };
    widgets::pill("cv-decompile", label, true, ready, p)
        .debug_selector(|| "cv-decompile".into())
        .when(ready, |el| {
            el.on_click(cx.listener(|this, _, _, cx| {
                this.state
                    .update(cx, |s, cx| s.decompile_selected(out_dir(), cx))
            }))
        })
        .into_any_element()
}
