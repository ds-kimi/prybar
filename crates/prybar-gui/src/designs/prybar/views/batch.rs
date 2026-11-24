//! The item list both batch screens share: progress, one row per model
//! or QC with its state, and "Open output" once something was written.

use gpui::{div, prelude::*, px, relative, AnyElement, Context, SharedString};

use super::super::widgets::Tipped as _;
use super::super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::batch::{BatchItem, BatchRun, ItemState};
use crate::theme::prybar::Tokens;

/// Rows drawn at most; the counts above still cover every item.
const MAX_ROWS: usize = 500;

pub fn view(id: &'static str, run: &BatchRun, t: Tokens, cx: &Context<Root>) -> AnyElement {
    if run.items.is_empty() {
        return div().into_any_element();
    }
    let (done, total) = (run.finished(), run.items.len());
    let share = done as f32 / total as f32;
    let bar = div().h(px(6.)).w_full().rounded_full().bg(t.raised);
    let bar = bar.child(div().h_full().w(relative(share)).rounded_full().bg(t.rust));
    div()
        .debug_selector(move || format!("{id}-items"))
        .flex()
        .flex_col()
        .gap_2()
        .child(widgets::section(
            format!("Items: {done} of {total} finished"),
            t,
        ))
        .child(bar)
        .child(rows(id, run, t))
        .child(open_output(id, run, t, cx))
        .into_any_element()
}

/// Failures first so a huge folder never hides what needs a look.
fn rows(id: &'static str, run: &BatchRun, t: Tokens) -> impl IntoElement {
    let mut items: Vec<(usize, &BatchItem)> = run.items.iter().enumerate().collect();
    items.sort_by_key(|(_, i)| !matches!(i.state, ItemState::Failed(_)));
    let more = items.len().saturating_sub(MAX_ROWS);
    let rows = items
        .into_iter()
        .take(MAX_ROWS)
        .map(|(ix, i)| row(id, ix, i, t));
    div()
        .flex()
        .flex_col()
        .child(
            div()
                .id(id)
                .max_h(px(260.))
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .children(rows),
        )
        .when(more > 0, |el| {
            el.child(div().text_color(t.muted).child(format!("and {more} more")))
        })
}

/// The state word in its color, then the name; hover says more.
fn row(id: &'static str, ix: usize, item: &BatchItem, t: Tokens) -> AnyElement {
    let (word, color, tip): (&str, _, SharedString) = match &item.state {
        ItemState::Queued => ("Queued", t.muted, "Waiting for a free worker".into()),
        ItemState::Running => ("Running", t.rust, "Working on it now".into()),
        ItemState::Done => ("Done", t.success, "Written to the output folder".into()),
        ItemState::Failed(why) => ("Failed", t.danger, why.clone().into()),
        ItemState::Cancelled => ("Cancelled", t.muted, "Skipped by Cancel".into()),
    };
    div()
        .id((id, ix))
        .debug_selector(move || format!("{id}-row-{ix}"))
        .flex()
        .gap_3()
        .px_1()
        .py(px(2.))
        .rounded_md()
        .hover(move |s| s.bg(t.hover))
        .tip(tip, t)
        .child(div().w(px(72.)).flex_none().text_color(color).child(word))
        .child(div().flex_1().min_w_0().truncate().child(item.key.clone()))
        .into_any_element()
}

fn open_output(id: &'static str, run: &BatchRun, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let Some(out) = run.out.clone().filter(|_| !run.running) else {
        return div().into_any_element();
    };
    widgets::button((id, usize::MAX), "Open output", Kind::Secondary, true, t)
        .debug_selector(move || format!("{id}-open"))
        .tip(format!("Show {} in the file manager", out.display()), t)
        .on_click(cx.listener(move |_, _, _, cx| cx.open_with_system(&out)))
        .into_any_element()
}
