//! Compile screen: QC, game, the compile button, and the log region.

use gpui::{div, prelude::*, px, AnyElement, Context};

use super::super::widgets::{self, Kind};
use super::preview::region;
use crate::app::{Forms, Root};
use crate::state::{AppState, Placeholder};
use crate::theme::prybar::Tokens;

/// Games offered until the picker reads the games config.
const GAMES: [&str; 4] = [
    "Garry's Mod",
    "Team Fortress 2",
    "Counter-Strike: Source",
    "Half-Life 2",
];

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_4()
        .child(form(s, f, t, cx))
        .child(region(Placeholder::CompileLog, t))
        .into_any_element()
}

/// Compile stays disabled until the log stream lands.
fn form(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    div()
        .max_w(px(520.))
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("Compile"))
        .child(
            div()
                .text_color(t.muted)
                .child("Build a QC with the game's studiomdl."),
        )
        .child(widgets::field("QC file", &f.qc, t).debug_selector(|| "pb-qc".into()))
        .child(widgets::section("Game", t))
        .child(games(s, t, cx))
        .child(div().flex().child(widgets::button(
            "pb-compile",
            "Compile",
            Kind::Primary,
            false,
            t,
        )))
}

fn games(s: &AppState, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let rows = GAMES.iter().enumerate().map(|(ix, name)| {
        widgets::row(("pb-game", ix), s.game == ix, t)
            .debug_selector(move || format!("pb-game-{ix}"))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| {
                    s.game = ix;
                    cx.notify();
                })
            }))
            .child(*name)
    });
    div().flex().flex_col().children(rows)
}
