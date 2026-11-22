//! The compile log: what studiomdl prints, as it prints it, with the
//! result on top and errors in the danger color.

use gpui::{div, prelude::*, AnyElement, ClipboardItem, Context, Rgba};

use super::super::super::widgets::Tipped as _;
use super::super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::state::tools::CompileOutcome;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let run = &s.tools.compile;
    if run.running {
        // Follow the tail while lines still arrive.
        f.compile_log.scroll_to_bottom();
    }
    div()
        .flex_1()
        .min_h_48()
        .flex()
        .flex_col()
        .gap_2()
        .child(header(s, t, cx))
        .child(lines(s, f, t))
        .into_any_element()
}

/// One row per line, colored by what studiomdl meant by it.
fn lines(s: &AppState, f: &Forms, t: Tokens) -> impl IntoElement {
    let run = &s.tools.compile;
    let rows = run.lines.iter().enumerate().map(|(ix, l)| {
        let color: Rgba = [t.text, t.rust, t.danger][l.kind as usize];
        div()
            .id(("pb-log-line", ix))
            .text_color(color)
            .child(l.text.clone())
    });
    let empty = run.lines.is_empty() && !run.running;
    let hint = "The studiomdl log shows here while a compile runs.";
    div()
        .id("pb-log")
        .debug_selector(|| "pb-log".into())
        .track_scroll(&f.compile_log)
        .overflow_y_scroll()
        .flex_1()
        .min_h_48()
        .p_3()
        .rounded_lg()
        .border_1()
        .border_color(t.border)
        .bg(t.surface)
        .font_family("Menlo")
        .text_xs()
        .when(empty, |el| el.text_color(t.muted).child(hint))
        .children(rows)
}

/// The result in one line, and Copy once there is a log to copy.
fn header(s: &AppState, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let run = &s.tools.compile;
    let (text, color) = match &run.outcome {
        _ if run.running => ("Compiling…".to_owned(), t.muted),
        None => (String::new(), t.muted),
        Some(o) => (
            o.describe(),
            if matches!(o, CompileOutcome::Built { .. }) {
                t.text
            } else {
                t.danger
            },
        ),
    };
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .child(
            div()
                .debug_selector(|| "pb-compile-result".into())
                .text_color(color)
                .child(text),
        )
        .child(copy(!run.lines.is_empty(), t, cx))
}

/// Copies the whole log, for a bug report or a forum post.
fn copy(has_log: bool, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    widgets::button("pb-log-copy", "Copy log", Kind::Secondary, has_log, t)
        .debug_selector(|| "pb-log-copy".into())
        .tip("Copy the whole log to the clipboard", t)
        .when(has_log, |el| {
            el.on_click(cx.listener(|this, _, _, cx| {
                let lines = &this.state.read(cx).tools.compile.lines;
                let text = lines
                    .iter()
                    .map(|l| l.text.as_str())
                    .collect::<Vec<_>>()
                    .join("\n");
                cx.write_to_clipboard(ClipboardItem::new_string(text));
            }))
        })
}
