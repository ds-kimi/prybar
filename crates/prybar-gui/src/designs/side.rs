//! Open documents and toasts, shown next to every screen.

use gpui::{div, prelude::*, AnyElement, Context};

use crate::app::Root;
use crate::state::{AppState, DesignKind, JobStatus, ToastKind};
use crate::theme::Palette;

/// Open files; clicking one makes it the target of every screen.
pub fn documents(
    titles: Vec<String>,
    selected: Option<usize>,
    p: Palette,
    cx: &mut Context<Root>,
) -> AnyElement {
    let rows: Vec<_> = titles
        .into_iter()
        .enumerate()
        .map(|(ix, title)| row(ix, title, selected == Some(ix), p, cx))
        .collect();
    div()
        .mt_4()
        .flex()
        .flex_col()
        .children(rows)
        .into_any_element()
}

fn row(
    ix: usize,
    title: String,
    current: bool,
    p: Palette,
    cx: &mut Context<Root>,
) -> impl IntoElement {
    div()
        .id(("doc", ix))
        .px_3()
        .py_1()
        .rounded_md()
        .text_sm()
        .cursor_pointer()
        .text_color(if current { p.accent } else { p.muted })
        .hover(move |s| s.bg(p.hover))
        .active(move |s| s.bg(p.active))
        .on_click(
            cx.listener(move |this, _, _, cx| this.state.update(cx, |s, cx| s.select(ix, cx))),
        )
        .child(title)
}

/// Design, screen and the jobs still running.
pub fn header(kind: DesignKind, state: &AppState) -> String {
    let running: Vec<&str> = state
        .jobs
        .iter()
        .filter(|j| j.status == JobStatus::Running)
        .map(|j| j.label.as_str())
        .collect();
    format!(
        "{} · {} · {}",
        kind.name(),
        state.screen.label(),
        running.join(", ")
    )
}

/// Newest last; errors in the danger color.
pub fn toasts(state: &AppState, p: Palette) -> AnyElement {
    let rows = state.toasts.iter().map(|t| {
        let color = match t.kind {
            ToastKind::Error => p.danger,
            ToastKind::Info => p.muted,
        };
        div()
            .id(("toast", t.id as usize))
            .text_sm()
            .text_color(color)
            .child(t.text.clone())
    });
    div()
        .mt_4()
        .flex()
        .flex_col()
        .gap_1()
        .children(rows)
        .into_any_element()
}
