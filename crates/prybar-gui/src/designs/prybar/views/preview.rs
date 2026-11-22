//! 3D and texture previews of the open model.

use std::path::PathBuf;

use gpui::{div, prelude::*, AnyElement};

use super::super::widgets;
use crate::app::Forms;
use crate::state::{AppState, Document, Screen};
use crate::theme::prybar::Tokens;

pub fn view(screen: Screen, doc: Option<&Document>, f: &Forms, t: Tokens) -> AnyElement {
    let name = doc.map_or_else(|| "No model open".into(), |d| d.title());
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
            (Screen::TexturePreview, Some(_)) => f.textures.clone().into_any_element(),
            _ => no_model(screen, t),
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

/// The model the texture screen should list, once it is open: opening
/// every game VPK is only worth it for people who look at textures.
pub fn wanted_textures(s: &AppState) -> Option<PathBuf> {
    (s.screen == Screen::TexturePreview)
        .then(|| super::pick(s, true))
        .flatten()
        .map(|d| d.path().to_owned())
}

/// Nothing to draw until a model is open; says how to get one.
fn no_model(screen: Screen, t: Tokens) -> AnyElement {
    let what = match screen {
        Screen::TexturePreview => "its materials",
        _ => "it in 3D",
    };
    div()
        .debug_selector(|| "pb-no-model".into())
        .flex_1()
        .min_h_48()
        .flex()
        .items_center()
        .justify_center()
        .rounded_lg()
        .border_1()
        .border_color(t.border)
        .bg(t.surface)
        .text_color(t.muted)
        .child(format!("Open a .mdl to see {what} here."))
        .into_any_element()
}
