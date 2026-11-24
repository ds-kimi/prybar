//! Compile screen: a QC or a folder of QCs, game, compile or cancel,
//! then the streamed log (one QC) or the item list (a folder).

mod input;
mod log;

use gpui::{div, prelude::*, px, AnyElement, Context};

use super::super::widgets::Tipped as _;
use super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::state::{AppState, Screen};
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_4()
        .child(form(s, f, t, cx))
        .child(super::batch::view("pb-cmp", &s.batch.compile, t, cx))
        .when(s.batch.compile.items.is_empty(), |el| {
            el.child(log::view(s, f, t, cx))
        })
        .into_any_element()
}

const INTRO: &str =
    "Build a QC, or every QC in a folder, with the game's studiomdl. Drop a .qc or a folder here.";
const NO_GAME: &str = "No game is set up yet. Detect games in Settings.";

fn form(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    div()
        .max_w(px(560.))
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("Compile"))
        .child(div().text_color(t.muted).child(INTRO))
        .child(input::view(s, f, t, cx))
        .child(widgets::section("Game", t))
        .child(games(s, t, cx))
        .child(div().flex().child(actions(s, f, t, cx)))
}

/// The configured games; with none, the way to get some.
fn games(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    if s.tools.games.is_empty() {
        let go = widgets::button("pb-to-settings", "Open Settings", Kind::Secondary, true, t)
            .tip("Add a game and its studiomdl path in Settings", t)
            .on_click(cx.listener(|this, _, _, cx| {
                this.state
                    .update(cx, |s, cx| s.navigate(Screen::Settings, cx))
            }));
        let why = div().text_color(t.muted).child(NO_GAME);
        return div()
            .flex()
            .items_center()
            .child(why)
            .child(go)
            .into_any_element();
    }
    let rows = s.tools.games.iter().enumerate().map(|(ix, g)| {
        widgets::row(("pb-game", ix), s.tools.game == ix, t)
            .debug_selector(move || format!("pb-game-{ix}"))
            .tip("Compile for this game", t)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| {
                    s.tools.game = ix;
                    cx.notify();
                })
            }))
            .child(g.name.clone())
    });
    div().flex().flex_col().children(rows).into_any_element()
}

/// Compile needs a real QC and a game; while it runs, Cancel takes over.
fn actions(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    if s.tools.compile.running || s.batch.compile.running {
        return widgets::button("pb-cancel", "Cancel", Kind::Secondary, true, t)
            .debug_selector(|| "pb-cancel".into())
            .tip(
                "Stop studiomdl; in a folder, the QCs not started yet are skipped",
                t,
            )
            .on_click(cx.listener(|this, _, _, cx| {
                this.state
                    .update(cx, |s, _| (s.cancel_compile(), s.cancel_batch()).1)
            }))
            .into_any_element();
    }
    let qc = f.qc.read(cx);
    let ready = !qc.text().trim().is_empty() && qc.error().is_none() && !s.tools.games.is_empty();
    widgets::button("pb-compile", "Compile", Kind::Primary, ready, t)
        .debug_selector(|| "pb-compile".into())
        .tip(
            "Run studiomdl on this QC, or on every QC in this folder, for the chosen game",
            t,
        )
        .when(ready, |el| {
            el.on_click(cx.listener(|this, _, _, cx| {
                let qc = this.forms.qc.read(cx).text().trim().into();
                this.state.update(cx, |s, cx| s.compile_input(qc, cx))
            }))
        })
        .into_any_element()
}
