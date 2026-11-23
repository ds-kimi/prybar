use gpui::{div, prelude::*, AnyElement, Context};

use super::super::super::super::widgets::Tipped as _;
use super::super::super::super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::tools::History;
use crate::theme::prybar::Tokens;
use prybar_workshop::changelog::Change;

/// The newest notes are the ones a creator checks; older ones are a
/// click away on the Workshop page.
const SHOWN: usize = 15;

pub fn view(h: Option<&History>, id: u64, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let small = |s: String| div().text_xs().text_color(t.muted).child(s).into_any_element();
    let retry = |why: String| retry(why, id, t, cx);
    let body = match h {
        None | Some(History::Waiting(_) | History::Loading(_)) => {
            vec![small("Reading the change history…".into())]
        }
        Some(History::Busy { wait, .. }) => {
            vec![retry(format!("Steam is busy, retrying in {wait} s."))]
        }
        Some(History::Failed(why)) => vec![retry(format!("No change history: {why}"))],
        Some(History::Ready(log)) => {
            let mut out = vec![];
            if log.stale || log.age >= 3600 {
                let why = if log.stale { "Steam did not answer; " } else { "" };
                out.push(small(format!("{why}copy read {}.", age(log.age))));
            }
            if log.changes.is_empty() {
                out.push(small("No change notes yet.".into()));
            }
            out.extend(log.changes.iter().take(SHOWN).map(|c| entry(c, t)));
            out
        }
    };
    div()
        .debug_selector(|| "pb-history".into())
        .flex()
        .flex_col()
        .gap_2()
        .child(widgets::section("Version history", t).pt_0())
        .children(body)
}

/// A calm line saying what happened, with a way to ask again now.
fn retry(why: String, id: u64, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let again = widgets::button("pb-hist-retry", "Try again", Kind::Secondary, true, t)
        .debug_selector(|| "pb-hist-retry".into())
        .tip("Ask Steam for the change notes now", t)
        .on_click(cx.listener(move |this, _, _, cx| {
            this.state.update(cx, |s, cx| s.retry_history(id, cx))
        }));
    div()
        .debug_selector(|| "pb-hist-busy".into())
        .flex()
        .items_center()
        .gap_2()
        .child(div().flex_1().text_xs().text_color(t.muted).child(why))
        .child(again)
        .into_any_element()
}

/// Coarse: "2 h ago" says enough about a cached page.
fn age(secs: u64) -> String {
    match secs {
        0..=3599 => "just now".to_owned(),
        3600..=86_399 => format!("{} h ago", secs / 3600),
        _ => format!("{} d ago", secs / 86_400),
    }
}

fn entry(c: &Change, t: Tokens) -> AnyElement {
    let note = match c.note.is_empty() {
        true => "(no note)".to_owned(),
        false => c.note.clone(),
    };
    div()
        .flex()
        .flex_col()
        .child(div().text_xs().child(c.headline.clone()))
        .child(div().text_xs().text_color(t.muted).child(note))
        .into_any_element()
}
