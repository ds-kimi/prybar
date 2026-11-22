//! 3D and texture previews, and the calm region every later-work part
//! uses: a labeled frame, no fake art, no motion.

use std::path::PathBuf;

use gpui::{div, prelude::*, AnyElement};

use super::super::widgets;
use crate::app::Forms;
use crate::state::{AppState, Document, Placeholder, Screen};
use crate::theme::prybar::Tokens;

pub fn view(screen: Screen, doc: Option<&Document>, f: &Forms, t: Tokens) -> AnyElement {
    let name = doc.map_or_else(|| "No model open".into(), |d| d.title());
    let ph = screen.placeholder().unwrap_or(Placeholder::ModelViewport);
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .items_baseline()
                .gap_3()
                .child(widgets::heading(screen.label()))
                .child(div().text_color(t.muted).child(name)),
        )
        .child(match (screen, doc) {
            (Screen::ModelPreview, Some(_)) => f.viewport.clone().into_any_element(),
            _ => region(ph, t),
        })
        .into_any_element()
}

/// The model the 3D view should show, once a screen with it is open:
/// the GPU only starts for people who look at a model.
pub fn wanted(s: &AppState) -> Option<PathBuf> {
    matches!(s.screen, Screen::Inspector | Screen::ModelPreview)
        .then(|| super::pick(s, true))
        .flatten()
        .map(|d| d.path().to_owned())
}

/// Says what will live here and that it is not built yet.
pub fn region(ph: Placeholder, t: Tokens) -> AnyElement {
    div()
        .debug_selector(|| "pb-placeholder".into())
        .flex_1()
        .min_h_48()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_1()
        .rounded_lg()
        .border_1()
        .border_color(t.border)
        .bg(t.surface)
        .child(ph.label())
        .child(
            div()
                .text_xs()
                .text_color(t.muted)
                .child("Coming in a later release"),
        )
        .into_any_element()
}
