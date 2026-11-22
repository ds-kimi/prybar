//! Settings: appearance and where the games config lives.

use gpui::{div, prelude::*, px, AnyElement};

use super::super::widgets;
use crate::actions::{ToggleTheme, MOD};
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, t: Tokens) -> AnyElement {
    div()
        .max_w(px(520.))
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("Settings"))
        .child(appearance(s, t))
        .child(widgets::section("Games", t))
        .child(widgets::field(
            "pb-config",
            "Games config",
            config_path(),
            t,
        ))
        .child(
            div()
                .text_color(t.muted)
                .child("Run `prybar games --detect --save` to fill it."),
        )
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

fn config_path() -> String {
    prybar_tools::config::config_path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "No config folder on this system".into())
}
