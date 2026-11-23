//! Keys inside the popup: Escape asks to close, Tab cycles its own fields.

use gpui::{prelude::*, Context, Div, MouseButton, Stateful, Window};

use super::Modal;
use crate::actions::{Back, FocusNext, FocusPrev};

/// More stops than any popup has; bounds the walk if focus never returns.
const MAX_STEPS: usize = 256;

impl Modal {
    /// Escape is the app's Back key; inside the popup it means close. A
    /// click inside never reaches the backdrop, which would close it.
    pub(super) fn keys(el: Stateful<Div>, cx: &mut Context<Self>) -> Stateful<Div> {
        el.on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_action(cx.listener(|this, _: &Back, _, cx| this.request_close(cx)))
            .on_action(cx.listener(|this, _: &FocusNext, w, cx| this.cycle(false, w, cx)))
            .on_action(cx.listener(|this, _: &FocusPrev, w, cx| this.cycle(true, w, cx)))
    }

    /// Moves focus one stop at a time until it lands inside the popup
    /// again, so Tab never reaches the screen underneath.
    pub(super) fn cycle(&mut self, back: bool, window: &mut Window, cx: &mut Context<Self>) {
        for _ in 0..MAX_STEPS {
            if back {
                window.focus_prev(cx);
            } else {
                window.focus_next(cx);
            }
            if self.focus.contains_focused(window, cx) && !self.focus.is_focused(window) {
                return;
            }
        }
        window.focus(&self.focus, cx);
    }
}
