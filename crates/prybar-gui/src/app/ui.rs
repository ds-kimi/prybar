//! Window-level toggles: command palette, theme variant, keyboard focus.

use gpui::{Context, Div, InteractiveElement as _, Stateful};

use super::root::Root;
use crate::actions::{FocusNext, FocusPrev, TogglePalette, ToggleTheme};

impl Root {
    pub(super) fn ui_actions(el: Stateful<Div>, cx: &mut Context<Self>) -> Stateful<Div> {
        el.on_action(cx.listener(|this, _: &TogglePalette, _, cx| {
            this.state.update(cx, |s, cx| {
                s.palette = !s.palette;
                cx.notify();
            })
        }))
        .on_action(cx.listener(|this, _: &ToggleTheme, _, cx| {
            this.state.update(cx, |s, cx| s.toggle_theme(cx))
        }))
        .on_action(cx.listener(|_, _: &FocusNext, window, cx| window.focus_next(cx)))
        .on_action(cx.listener(|_, _: &FocusPrev, window, cx| window.focus_prev(cx)))
    }
}
