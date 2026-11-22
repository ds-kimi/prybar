//! Window-level toggles: command palette, theme variant, keyboard focus.

use gpui::{Context, Div, InteractiveElement as _, KeyDownEvent, Stateful};

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
            this.state.update(cx, |s, cx| {
                s.light = !s.light;
                cx.notify();
            })
        }))
        .on_action(cx.listener(|_, _: &FocusNext, window, cx| window.focus_next(cx)))
        .on_action(cx.listener(|_, _: &FocusPrev, window, cx| window.focus_prev(cx)))
        .on_key_down(cx.listener(|this, ev: &KeyDownEvent, _, cx| this.type_search(ev, cx)))
    }

    /// While search is open, plain typing edits the query; shortcuts
    /// with cmd or ctrl still reach their bindings.
    fn type_search(&mut self, ev: &KeyDownEvent, cx: &mut Context<Self>) {
        let m = &ev.keystroke.modifiers;
        if m.platform || m.control {
            return;
        }
        let key = ev.keystroke.key.as_str();
        let typed = ev.keystroke.key_char.clone();
        self.state.update(cx, |s, cx| {
            let Some(query) = s.search.as_mut() else {
                return;
            };
            match (key, typed) {
                ("backspace", _) => {
                    query.pop();
                }
                (_, Some(c)) if !c.chars().any(char::is_control) => query.push_str(&c),
                _ => return,
            }
            cx.notify();
        });
    }
}
