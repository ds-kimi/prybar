//! Which Workshop game the page publishes to: one chip per supported
//! game installed here, or what to do when none is.

use gpui::{div, prelude::*, AnyElement, Context};
use prybar_workshop::games::{WorkshopGame, WORKSHOP_GAMES};

use super::super::super::widgets::{self, Kind, Tipped as _};
use crate::app::Root;
use crate::state::{AppState, Screen};
use crate::theme::prybar::Tokens;

/// Chips instead of a dropdown: two games at most, both always in
/// sight. A single installed game still shows, so the user knows where
/// items go.
pub fn chips(s: &AppState, t: Tokens, cx: &Context<Root>) -> Option<AnyElement> {
    let games = s
        .tools
        .steam
        .games
        .as_ref()
        .filter(|g| !g.detected.is_empty())?;
    let chips = games.detected.iter().enumerate().map(|(i, g)| {
        let on = s.tools.steam.game == Some(g.app_id);
        chip(i, g, on, t, cx)
    });
    let row = div()
        .debug_selector(|| "pb-games".into())
        .flex()
        .flex_wrap()
        .items_center()
        .gap_1()
        .child(div().text_sm().text_color(t.muted).mr_1().child("Game"))
        .children(chips);
    Some(row.into_any_element())
}

fn chip(i: usize, g: &WorkshopGame, on: bool, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let app = g.app_id;
    let tip = format!("Publish to {} (app {app}): {}", g.name, g.rules);
    widgets::ring(div().id(("pb-game", i)), t)
        .debug_selector(move || format!("pb-game-{i}"))
        .h_7()
        .px_3()
        .flex()
        .items_center()
        .rounded_md()
        .text_sm()
        .cursor_pointer()
        .bg(if on { t.active } else { t.surface })
        .text_color(if on { t.rust } else { t.text })
        .hover(move |st| st.bg(t.hover))
        .tip(tip, t)
        .on_click(
            cx.listener(move |this, _, _, cx| this.state.update(cx, |s, cx| s.pick_game(app, cx))),
        )
        .child(g.name)
}

/// Says which games work and where to add one, since the cause is
/// almost always a library Steam detection did not find.
pub fn empty(t: Tokens, cx: &Context<Root>) -> AnyElement {
    let open = widgets::button(
        "pb-games-settings",
        "Open Settings",
        Kind::Secondary,
        true,
        t,
    )
    .debug_selector(|| "pb-games-settings".into())
    .tip("Detect games or set a game folder by hand", t)
    .on_click(cx.listener(|this, _, _, cx| {
        this.state
            .update(cx, |s, cx| s.navigate(Screen::Settings, cx))
    }));
    let card = div()
        .p_4()
        .flex()
        .flex_col()
        .items_start()
        .gap_3()
        .rounded_lg();
    card.debug_selector(|| "pb-games-empty".into())
        .border_1()
        .border_color(t.border)
        .bg(t.surface)
        .child(div().text_color(t.text).child(empty_text()))
        .child(open)
        .into_any_element()
}

fn empty_text() -> String {
    let names: Vec<&str> = WORKSHOP_GAMES.iter().map(|g| g.name).collect();
    format!(
        "No supported Workshop game is installed here. prybar publishes to {}. \
         Install one in Steam, or add its folder in Settings.",
        names.join(" and ")
    )
}
