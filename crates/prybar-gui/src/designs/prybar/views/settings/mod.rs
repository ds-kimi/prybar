//! Settings: appearance, and the games config (detect, edit, save).

mod games;

use gpui::{div, prelude::*, px, AnyElement, Context};

use super::super::widgets::{self, Kind};
use crate::actions::{ToggleTheme, MOD};
use crate::app::{Forms, Root};
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let path = prybar_tools::config::config_path()
        .map_or_else(|e| e.to_string(), |p| p.display().to_string());
    let busy = s.is_running("Detect games");
    let detect = widgets::button(
        "pb-detect",
        if busy { "Detecting…" } else { "Detect games" },
        Kind::Secondary,
        !busy,
        t,
    )
    .debug_selector(|| "pb-detect".into())
    .when(!busy, |el| {
        el.on_click(cx.listener(|this, _, _, cx| this.state.update(cx, |s, cx| s.detect_games(cx))))
    });
    div()
        .max_w(px(520.))
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("Settings"))
        .child(appearance(s, t))
        .child(widgets::section("Games", t))
        .child(widgets::readonly("Games config", path, t))
        .child(div().flex().child(detect))
        .child(games::list(s, f, t, cx))
        .child(games::editor(s, f, t, cx))
        .into_any_element()
}

fn appearance(s: &AppState, t: Tokens) -> impl IntoElement {
    let theme = widgets::switch("pb-light", "Light theme", s.light, t)
        .debug_selector(|| "pb-light".into())
        .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleTheme), cx));
    let hint = format!("Also {MOD}-shift-l. Saved for next time.");
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(widgets::section("Appearance", t))
        .child(theme)
        .child(div().text_xs().text_color(t.muted).child(hint))
}
