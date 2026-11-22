//! Window-level toggles: command palette, theme variant, keyboard focus.

use gpui::{Context, Div, Entity, InteractiveElement as _, Stateful, Window};

use super::root::Root;
use crate::actions::{FocusNext, FocusPrev, TogglePalette, ToggleTheme};
use crate::designs::modal::{Closed, Modal};

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

    /// Shows a popup over everything and moves focus into it; when it
    /// closes, focus comes back to the window.
    // The publish popups call it; until they land only tests do.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn show_modal(
        &mut self,
        modal: Entity<Modal>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let sub = cx.subscribe_in(&modal, window, |this, _, _: &Closed, window, cx| {
            this.modal = None;
            this.focus.focus(window, cx);
            cx.notify()
        });
        modal.read(cx).focus_handle().clone().focus(window, cx);
        self.modal = Some((modal, sub));
        cx.notify();
    }
}
