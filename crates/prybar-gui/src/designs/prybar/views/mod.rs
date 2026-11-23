//! The area under the bar: one view per screen, on one calm frame of a
//! title, a line saying what the screen is for, then its content.

mod compile;
mod decompile;
mod home;
mod model;
mod package;
mod preview;
mod privacy;
mod publish;
mod settings;

use gpui::{div, prelude::*, AnyElement, Context};

pub use preview::{wanted, wanted_textures};
pub use privacy::notice;
pub use publish::{footer as publish_footer, popup as publish_popup};

use super::motion;
use super::widgets::Tipped as _;
use super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::state::{AppState, Document, Screen};
use crate::theme::prybar::Tokens;

pub fn screen(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let ix = Screen::ALL.iter().position(|x| *x == s.screen).unwrap_or(0);
    let body = div()
        .id("pb-view")
        .debug_selector(|| "pb-view".into())
        .size_full()
        .px_6()
        .py_5()
        .flex()
        .flex_col()
        .gap_4()
        // The Workshop page scrolls its grid and panel, never itself:
        // two nested scrollers would fight under the wheel.
        .when(s.screen != Screen::Publish, |el| el.overflow_y_scroll())
        .when(s.screen == Screen::Publish, |el| el.overflow_hidden())
        .child(content(s, f, t, cx));
    motion::enter(div().flex_1().min_h_0().child(body), ("pb-view-in", ix))
}

/// Model and package screens follow the selection when it fits, else
/// the last opened file of their kind, so switching tabs never blanks them.
fn pick(s: &AppState, model: bool) -> Option<&Document> {
    let fits = |d: &&Document| matches!(d, Document::Model { .. }) == model;
    let selected = s.selected.and_then(|i| s.documents.get(i)).filter(fits);
    selected.or_else(|| s.documents.iter().rev().find(fits))
}

fn content(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    match s.screen {
        Screen::Home => home::view(s, t, cx),
        Screen::Inspector => {
            pick(s, true).map_or_else(|| empty(s.screen, t, cx), |d| model::view(d, f, t, cx))
        }
        Screen::Package => match pick(s, false) {
            Some(Document::Package { report, .. }) => {
                package::view(report, s.search.as_deref(), f, t, cx)
            }
            _ => empty(s.screen, t, cx),
        },
        Screen::Decompile => decompile::view(s, f, t, cx),
        Screen::Compile => compile::view(s, f, t, cx),
        Screen::Publish => publish::view(s, f, t, cx),
        Screen::Settings => settings::view(s, f, t, cx),
        Screen::ModelPreview | Screen::TexturePreview => {
            preview::view(s.screen, pick(s, true), f, t)
        }
    }
}

/// A screen that needs a file of a kind that is not open yet.
fn empty(screen: Screen, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let want = match screen {
        Screen::Package => "Open a VPK, GMA, APK or HFS package to browse its files.",
        _ => "Open a .mdl to see its bones, meshes, sequences and materials.",
    };
    let go = widgets::button("pb-empty-open", "Open file", Kind::Primary, true, t)
        .debug_selector(|| "pb-empty-open".into())
        .tip("Pick a file to open", t)
        .on_click(cx.listener(|this, _, _, cx| {
            this.state.update(cx, |s, cx| s.navigate(Screen::Home, cx))
        }));
    div()
        .flex()
        .flex_col()
        .items_start()
        .gap_3()
        .child(widgets::heading(screen.label()))
        .child(div().text_color(t.muted).child(want))
        .child(go)
        .into_any_element()
}
