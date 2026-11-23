use gpui::{div, prelude::*};

use super::super::super::super::widgets;
use crate::state::tools::History;
use crate::theme::prybar::Tokens;
use prybar_workshop::changelog::Change;

/// The newest notes are the ones a creator checks; older ones are a
/// click away on the Workshop page.
const SHOWN: usize = 15;

pub fn view(h: Option<&History>, t: Tokens) -> impl IntoElement {
    let small = |s: String| div().text_xs().text_color(t.muted).child(s);
    let body = match h {
        None | Some(History::Loading) => vec![small("Reading the change history…".into())],
        Some(History::Failed(why)) => vec![small(format!("No change history: {why}"))],
        Some(History::Ready(all)) if all.is_empty() => vec![small("No change notes yet.".into())],
        Some(History::Ready(all)) => all.iter().take(SHOWN).map(|c| entry(c, t)).collect(),
    };
    div()
        .debug_selector(|| "pb-history".into())
        .flex()
        .flex_col()
        .gap_2()
        .child(widgets::section("Version history", t).pt_0())
        .children(body)
}

fn entry(c: &Change, t: Tokens) -> gpui::Div {
    let note = match c.note.is_empty() {
        true => "(no note)".to_owned(),
        false => c.note.clone(),
    };
    div()
        .flex()
        .flex_col()
        .child(div().text_xs().child(c.headline.clone()))
        .child(div().text_xs().text_color(t.muted).child(note))
}
