//! Compile and settings. Later-work parts are animated
//! placeholders inside a real layout.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::super::placeholder;
use super::super::widgets;
use crate::app::Root;
use crate::state::{AppState, Placeholder};
use crate::theme::Palette;

/// Games offered by the compile picker until games config is wired.
const GAMES: [&str; 4] = [
    "Garry's Mod",
    "Team Fortress 2",
    "Counter-Strike: Source",
    "Half-Life 2",
];

pub fn compile(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(widgets::title(
            "Compile",
            "Turn a .qc back into a model for a game.",
            p,
        ))
        .child(widgets::chip("No QC chosen yet", false, p))
        .child(games(s, p, cx))
        .child(
            div()
                .flex()
                .child(widgets::pill("cv-compile", "Compile it", true, false, p)),
        )
        .child(placeholder::region(Placeholder::CompileLog, p))
        .into_any_element()
}

/// One card per game; the picked one is filled in the accent.
fn games(s: &AppState, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let cards = GAMES.iter().enumerate().map(|(ix, name)| {
        let on = s.game == ix;
        widgets::card(("cv-game", ix), p)
            .debug_selector(move || format!("cv-game-{ix}"))
            .w_48()
            .when(on, |el| {
                el.bg(p.accent).text_color(p.surface).border_color(p.accent)
            })
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| {
                    s.game = ix;
                    cx.notify();
                })
            }))
            .child(*name)
    });
    div().flex().flex_wrap().gap_3().children(cards)
}

pub fn settings(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let split = widgets::card("cv-set-split", p)
        .debug_selector(|| "cv-set-split".into())
        .flex()
        .items_center()
        .gap_3()
        .on_click(cx.listener(|this, _, _, cx| {
            this.state.update(cx, |s, cx| {
                s.split_qc = !s.split_qc;
                cx.notify();
            })
        }))
        .child(widgets::chip(
            if s.split_qc { "On" } else { "Off" },
            s.split_qc,
            p,
        ))
        .child("Split decompiled QC into one .qci per section");
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(widgets::title("Settings", "How prybar works for you.", p))
        .child(split)
        .child(widgets::title("Games", "Detected games appear here.", p))
        .child(widgets::pill("cv-detect", "Find my games", false, false, p).w_48())
        .into_any_element()
}
