//! Compile: a game picker over the later-work log stream.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::super::placeholder;
use super::super::widgets;
use crate::app::Root;
use crate::state::{AppState, Placeholder};
use crate::theme::Palette;

/// Games offered by the compile picker until games config is wired.
const GAMES: [&str; 4] = ["garrysmod", "tf", "cstrike", "hl2"];

pub fn compile(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("compile", "qc: none chosen", p))
        .child(games(s, p, cx))
        .child(div().flex().child(widgets::action(
            "cs-compile",
            "compile",
            "compile",
            false,
            p,
        )))
        .child(placeholder::region(Placeholder::CompileLog, p))
        .into_any_element()
}

/// `(*)` marks the picked game.
fn games(s: &AppState, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let games = GAMES.iter().enumerate().map(|(ix, name)| {
        let on = s.game == ix;
        widgets::row(("cs-game", ix), p)
            .debug_selector(move || format!("cs-game-{ix}"))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| {
                    s.game = ix;
                    cx.notify();
                })
            }))
            .child(
                div()
                    .text_color(p.accent)
                    .child(if on { "(*)" } else { "( )" }),
            )
            .child(*name)
    });
    div().flex().flex_col().children(games)
}
