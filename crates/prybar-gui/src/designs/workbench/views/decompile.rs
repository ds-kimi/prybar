//! Decompile screen: target, options, run, and the last result.

use std::path::PathBuf;

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::widgets;
use crate::app::Root;
use crate::state::{AppState, JobStatus};
use crate::theme::Palette;

/// Output goes to a temp folder until a folder picker lands.
fn out_dir() -> PathBuf {
    std::env::temp_dir()
        .join("prybar-gui-out")
        .join("decompile")
}

pub fn view(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let target = s
        .selected
        .and_then(|i| s.documents.get(i))
        .map(|d| d.title());
    let split = widgets::toggle("wb-split", "One .qci per QC section", s.split_qc, p)
        .debug_selector(|| "wb-split".into())
        .on_click(cx.listener(|this, _, _, cx| {
            this.state.update(cx, |s, cx| {
                s.split_qc = !s.split_qc;
                cx.notify();
            })
        }));
    let input = target.clone().unwrap_or_else(|| "Open a file first".into());
    let out = out_dir().display().to_string();
    div()
        .flex()
        .flex_col()
        .gap_3()
        .max_w_96()
        .child(div().text_lg().child("Decompile"))
        .child(widgets::field("wb-target", "Input", input, p))
        .child(widgets::field("wb-out", "Output folder", out, p))
        .child(split)
        .child(div().flex().child(run_button(s, target.is_some(), p, cx)))
        .child(result(s, p))
        .into_any_element()
}

/// Disabled with no file, and while a decompile is already running.
fn run_button(s: &AppState, has_target: bool, p: Palette, cx: &Context<Root>) -> AnyElement {
    let busy = s
        .jobs
        .iter()
        .any(|j| j.label == "Decompile" && j.status == JobStatus::Running);
    let ready = has_target && !busy;
    let label = if busy { "Running…" } else { "Decompile" };
    widgets::button("wb-decompile", label, true, ready, p)
        .debug_selector(|| "wb-decompile".into())
        .when(ready, |el| {
            el.on_click(cx.listener(|this, _, _, cx| {
                this.state
                    .update(cx, |s, cx| s.decompile_selected(out_dir(), cx))
            }))
        })
        .into_any_element()
}

fn result(s: &AppState, p: Palette) -> AnyElement {
    let Some(r) = &s.last_decompile else {
        return div().into_any_element();
    };
    let ok = r.models.iter().map(|m| {
        div()
            .flex()
            .gap_2()
            .child(div().text_color(p.accent).child("✓"))
            .child(format!(
                "{} → {} ({} files)",
                m.input,
                m.out_dir.display(),
                m.files
            ))
    });
    let bad = r.failures.iter().map(|f| {
        div()
            .text_color(p.danger)
            .child(format!("✗ {}: {}", f.input, f.error))
    });
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(widgets::section("Last run", p))
        .children(ok)
        .children(bad)
        .into_any_element()
}
