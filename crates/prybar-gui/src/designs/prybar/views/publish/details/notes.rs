use gpui::{div, prelude::*, AnyElement};
use prybar_workshop::changelog::{Change, Log};

use crate::theme::prybar::Tokens;

/// The newest notes are the ones a creator checks; older ones are a
/// click away on the Workshop page.
const SHOWN: usize = 15;

/// The notes, after a line on how old the copy is when that matters.
pub fn view(log: &Log, t: Tokens) -> Vec<AnyElement> {
    let small = |s: String| {
        div()
            .text_xs()
            .text_color(t.muted)
            .child(s)
            .into_any_element()
    };
    let mut out = vec![];
    if log.stale || log.age >= 3600 {
        let why = if log.stale {
            "Steam did not answer; "
        } else {
            ""
        };
        out.push(small(format!("{why}copy read {}.", age(log.age))));
    }
    if log.changes.is_empty() {
        out.push(small("No change notes yet.".into()));
    }
    out.extend(log.changes.iter().take(SHOWN).map(|c| entry(c, t)));
    out
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
