//! The games list and the editor for one game's folders.

use gpui::{div, prelude::*, AnyElement, Context, Entity};
use prybar_service::tools::Game;

use super::super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::designs::input::TextInput;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

/// Picking a game loads its folders into the fields below.
pub fn list(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    if s.tools.games.is_empty() {
        return div()
            .text_color(t.muted)
            .child("No games yet. Detect finds Steam installs of Source games.")
            .into_any_element();
    }
    let rows = s.tools.games.iter().enumerate().map(|(ix, g)| {
        let (dir, tool) = (
            g.game_dir.display().to_string(),
            g.studiomdl
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_default(),
        );
        let (dir_in, tool_in) = (f.game_dir.clone(), f.studiomdl.clone());
        widgets::row(("pb-set-game", ix), s.tools.editing == Some(ix), t)
            .debug_selector(move || format!("pb-set-game-{ix}"))
            .on_click(cx.listener(move |this, _, _, cx| {
                dir_in.update(cx, |i, cx| i.set_text(&dir, cx));
                tool_in.update(cx, |i, cx| i.set_text(&tool, cx));
                this.state.update(cx, |s, cx| {
                    s.tools.editing = Some(ix);
                    cx.notify();
                })
            }))
            .child(g.name.clone())
    });
    div().flex().flex_col().children(rows).into_any_element()
}

/// The picked game's folder and studiomdl, saved back to the config.
pub fn editor(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let Some(game) = s.tools.editing.and_then(|i| s.tools.games.get(i)) else {
        return div().into_any_element();
    };
    let ok = f.game_dir.read(cx).error().is_none() && f.studiomdl.read(cx).error().is_none();
    let save = save(game.name.clone(), ok, t, cx);
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            widgets::field("Game folder", &f.game_dir, t).debug_selector(|| "pb-game-dir".into()),
        )
        .child(widgets::field("studiomdl", &f.studiomdl, t))
        .child(
            div()
                .text_xs()
                .text_color(t.muted)
                .child("Leave studiomdl empty to keep the detected one."),
        )
        .child(div().flex().child(save))
        .into_any_element()
}

/// Empty studiomdl keeps the detected one: config_set merges it.
fn save(name: String, ok: bool, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    widgets::button("pb-save-game", "Save", Kind::Primary, ok, t)
        .debug_selector(|| "pb-save-game".into())
        .when(ok, |el| {
            el.on_click(cx.listener(move |this, _, _, cx| {
                let text = |e: &Entity<TextInput>| e.read(cx).text().trim().to_owned();
                let (dir, tool) = (text(&this.forms.game_dir), text(&this.forms.studiomdl));
                let game = Game {
                    name: name.clone(),
                    game_dir: dir.into(),
                    studiomdl: Some(tool).filter(|t| !t.is_empty()).map(Into::into),
                    hlmv: None,
                    packer: None,
                };
                this.state.update(cx, |s, cx| s.save_game(game, cx))
            }))
        })
}
