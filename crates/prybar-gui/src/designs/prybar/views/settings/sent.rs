//! "What was sent": one row per log line, newest first; a click shows
//! the exact JSON that was queued or sent.

use gpui::{div, prelude::*, AnyElement, Context};
use prybar_telemetry::{Entry, Event};

use super::super::super::widgets;
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

/// Older rows stay in the file; the screen only needs the recent ones.
const SHOWN: usize = 50;

pub fn table(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let log = &s.privacy.log;
    let mut list = div().flex().flex_col().gap_1().child(
        div()
            .text_xs()
            .text_color(t.muted)
            .child("What was sent (time UTC, event, status, size)"),
    );
    if log.is_empty() {
        list = list.child(div().text_sm().text_color(t.muted).child("Nothing yet."));
    }
    for i in (0..log.len()).rev().take(SHOWN) {
        list = list.children(entry(s, i, t, cx));
    }
    list.into_any_element()
}

/// The row, plus the payload as pretty JSON under it when opened.
fn entry(s: &AppState, i: usize, t: Tokens, cx: &Context<Root>) -> Vec<AnyElement> {
    let e = &s.privacy.log[i];
    let open = s.privacy.open == Some(i);
    let row = widgets::row(("pb-sent", i), open, t)
        .debug_selector(move || format!("pb-sent-{i}"))
        .child(line(e))
        .on_click(cx.listener(move |this, _, _, cx| {
            this.state.update(cx, |s, cx| {
                s.privacy.open = (s.privacy.open != Some(i)).then_some(i);
                cx.notify();
            })
        }));
    let mut out = vec![row.into_any_element()];
    if open {
        let json = div()
            .p_2()
            .rounded_md()
            .bg(t.surface)
            .font_family("Menlo")
            .text_xs()
            .child(e.payload.json(true));
        out.push(json.into_any_element());
    }
    out
}

fn line(e: &Entry) -> String {
    let name = match e.payload {
        Event::AppStart { .. } => "app_start",
        Event::Crash { .. } => "crash",
    };
    let size = e.payload.json(false).len();
    format!("{}  {name}  {}  {size} B", when(e.time), e.status)
}

/// `YYYY-MM-DD HH:MM` in UTC from Unix seconds, without a date crate.
fn when(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let z = days + 719_468;
    let (era, doe) = (z.div_euclid(146_097), z.rem_euclid(146_097));
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    let (h, min) = (secs % 86_400 / 3600, secs % 3600 / 60);
    format!("{y:04}-{m:02}-{d:02} {h:02}:{min:02}")
}
