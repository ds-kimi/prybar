//! The frame around every screen: sidebar, tabs, inspector, status bar
//! and the command palette.

pub mod inspector;
pub mod palette;
pub mod status;
pub mod tabs;

use gpui::{div, prelude::*, px, AnyElement, Context};

use super::{motion, widgets};
use crate::app::Root;
use crate::state::{AppState, Screen};
use crate::theme::Palette;

/// Height of one sidebar row; the selection highlight moves in steps of it.
const ROW: f32 = 28.;

/// Screens first, then the open files, which also live in the tabs.
pub fn sidebar(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    div()
        .w(px(208.))
        .flex_none()
        .p_2()
        .flex()
        .flex_col()
        .bg(p.surface)
        .border_r_1()
        .border_color(p.border)
        .child(widgets::section("Screens", p))
        .child(nav_list(s, p, cx))
        .child(widgets::section("Files", p))
        .child(tabs::file_list(s, p, cx))
        .into_any_element()
}

/// The highlight sits under the rows and springs to the current one.
fn nav_list(s: &AppState, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let current = Screen::ALL.iter().position(|x| *x == s.screen).unwrap_or(0);
    let highlight = div()
        .absolute()
        .left_0()
        .right_0()
        .h(px(ROW))
        .rounded_sm()
        .bg(p.active)
        .border_l_2()
        .border_color(p.accent);
    let rows = Screen::ALL
        .into_iter()
        .map(|screen| nav(screen, screen == s.screen, p, cx));
    div()
        .relative()
        .child(motion::slide_to(
            highlight,
            "wb-nav-hl",
            current as f32 * ROW,
            false,
        ))
        .children(rows)
}

fn nav(screen: Screen, current: bool, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let label = screen.label();
    widgets::row(label, current, p)
        .debug_selector(move || format!("nav-{}", label.to_lowercase()))
        .on_click(
            cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| s.navigate(screen, cx))
            }),
        )
        .child(label)
}
