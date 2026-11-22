//! Compile screen: QC, game, compile or cancel, then the streamed log.

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
        .child(log::view(s, f, t, cx))
        .into_any_element()
}

fn form(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let choose = widgets::button("pb-qc-pick", "Choose…", Kind::Secondary, true, t)
        .tip("Pick the .qc file to compile", t)
        .on_click(cx.listener(|this, _, _, cx| this.pick_into(this.forms.qc.clone(), false, cx)));
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
        .child(
            div()
                .flex()
                .items_end()
                .gap_2()
                .child(
                    widgets::field("QC file", &f.qc, t)
                        .flex_1()
                        .debug_selector(|| "pb-qc".into()),
                )
                .child(choose),
        )
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
        let why = div()
            .text_color(t.muted)
            .child("No game is set up yet. Detect games in Settings.");
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
    let run = &s.tools.compile;
    if run.running {
        return widgets::button("pb-cancel", "Cancel", Kind::Secondary, true, t)
            .debug_selector(|| "pb-cancel".into())
            .tip("Stop studiomdl", t)
            .on_click(
                cx.listener(|this, _, _, cx| this.state.update(cx, |s, _| s.cancel_compile())),
            )
            .into_any_element();
    }
    let qc = f.qc.read(cx);
    let ready = !qc.text().trim().is_empty() && qc.error().is_none() && !s.tools.games.is_empty();
    widgets::button("pb-compile", "Compile", Kind::Primary, ready, t)
        .debug_selector(|| "pb-compile".into())
        .tip("Run studiomdl on this QC for the chosen game", t)
        .when(ready, |el| {
            el.on_click(cx.listener(|this, _, _, cx| {
                let qc = this.forms.qc.read(cx).text().trim().into();
                this.state.update(cx, |s, cx| s.compile(qc, cx))
            }))
        })
        .into_any_element()
}
