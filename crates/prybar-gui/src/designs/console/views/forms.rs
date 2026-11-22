//! Decompile and settings: options as toggled rows, runs as
//! bracketed actions, results as plain lines.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::widgets;
use super::out_dir;
use crate::app::Root;
use crate::state::{AppState, JobStatus};
use crate::theme::Palette;

pub fn decompile(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let target = s
        .selected
        .and_then(|i| s.documents.get(i))
        .map(|d| d.title());
    let out = format!("out {}", out_dir("decompile").display());
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading(
            "decompile",
            target.clone().unwrap_or_else(|| "no file".into()),
            p,
        ))
        .child(div().text_color(p.muted).child(out))
        .child(split(s.split_qc, p, cx))
        .child(div().flex().child(run(s, target.is_some(), p, cx)))
        .child(super::more::result(s, p))
        .into_any_element()
}

/// Inert with no file and while a decompile is already running.
fn run(s: &AppState, has_target: bool, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let busy = s
        .jobs
        .iter()
        .any(|j| j.label == "Decompile" && j.status == JobStatus::Running);
    let ready = has_target && !busy;
    let label = if busy { "working…" } else { "decompile" };
    widgets::action("cs-decompile", label, "decompile", ready, p).when(ready, |el| {
        el.on_click(cx.listener(|this, _, _, cx| {
            this.state
                .update(cx, |s, cx| s.decompile_selected(out_dir("decompile"), cx))
        }))
    })
}

/// `[x]` toggle row for the split-QC option.
pub fn split(on: bool, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    widgets::row("cs-split", p)
        .debug_selector(|| "cs-split".into())
        .on_click(cx.listener(|this, _, _, cx| {
            this.state.update(cx, |s, cx| {
                s.split_qc = !s.split_qc;
                cx.notify();
            })
        }))
        .child(
            div()
                .text_color(p.accent)
                .child(if on { "[x]" } else { "[ ]" }),
        )
        .child("split-qc  one .qci per QC section")
        .child(widgets::key("toggle split-qc", p))
}

pub fn settings(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let detect = widgets::action("cs-detect", "detect games", "games --detect", false, p);
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("settings", "defaults for every run", p))
        .child(split(s.split_qc, p, cx))
        .child(widgets::heading("games", "detected games appear here", p))
        .child(div().flex().child(detect))
        .into_any_element()
}
