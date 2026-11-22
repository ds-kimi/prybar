//! The frame: one horizontal top bar holding the wordmark, the screens,
//! job status, the theme toggle and settings; toasts and the command
//! palette float above.

pub mod palette;
mod status;
pub mod toasts;

use gpui::{div, prelude::*, AnyElement, Context, FontWeight};

use super::widgets::{self, Kind};
use crate::actions::ToggleTheme;
use crate::app::Root;
use crate::state::{AppState, Screen};
use crate::theme::prybar::Tokens;

/// Screens in the bar, in work order. Settings sits apart on the right;
/// the two previews are reached from the model screen and the palette.
const MAIN: [Screen; 6] = [
    Screen::Home,
    Screen::Inspector,
    Screen::Package,
    Screen::Decompile,
    Screen::Compile,
    Screen::Publish,
];

pub fn bar(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let items = MAIN.into_iter().map(|screen| nav(screen, s, t, cx));
    let brand = div()
        .pr_3()
        .font_weight(FontWeight::SEMIBOLD)
        .child("prybar");
    div()
        .id("pb-nav")
        .debug_selector(|| "pb-nav".into())
        .flex_none()
        .relative()
        .h_12()
        .px_4()
        .flex()
        .items_center()
        .gap_1()
        .bg(t.surface)
        .border_b_1()
        .border_color(t.border)
        .child(brand)
        .children(items)
        .child(div().flex_1())
        .children(status::job(s, t))
        .child(theme_toggle(s.light, t, cx))
        .child(nav(Screen::Settings, s, t, cx))
        .children(status::sweep(s, t))
        .into_any_element()
}

fn nav(screen: Screen, s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let label = screen.label();
    widgets::nav_item(label, label, screen == s.screen, t)
        .debug_selector(move || format!("nav-{}", label.to_lowercase()))
        .on_click(
            cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| s.navigate(screen, cx))
            }),
        )
        .into_any_element()
}

/// Names the theme it switches to, so the label is the action.
fn theme_toggle(light: bool, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let label = if light { "Dark" } else { "Light" };
    widgets::button("pb-theme", label, Kind::Ghost, true, t)
        .debug_selector(|| "pb-theme".into())
        .on_click(cx.listener(|_, _, window, cx| window.dispatch_action(Box::new(ToggleTheme), cx)))
        .into_any_element()
}
