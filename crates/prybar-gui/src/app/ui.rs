//! Window-level toggles: command palette, theme variant, keyboard focus,
//! and typing into the search or command bar.

use gpui::{Context, Div, InteractiveElement as _, KeyDownEvent, Stateful};

use super::root::Root;
use crate::actions::{
    FocusNext, FocusPrev, SelectNext, SelectPrev, Submit, TogglePalette, ToggleTheme,
};
use crate::designs::console::run;
use crate::state::DesignKind;

impl Root {
    pub(super) fn ui_actions(el: Stateful<Div>, cx: &mut Context<Self>) -> Stateful<Div> {
        el.on_action(cx.listener(|this, _: &TogglePalette, _, cx| {
            this.state.update(cx, |s, cx| {
                // The console's palette is its command bar, which is the search field.
                if s.design == Some(DesignKind::Console) {
                    s.search = s.search.is_none().then(String::new);
                    s.cursor = 0;
                } else {
                    s.palette = !s.palette;
                }
                cx.notify();
            })
        }))
        .on_action(cx.listener(|this, _: &ToggleTheme, _, cx| {
            this.state.update(cx, |s, cx| s.toggle_theme(cx))
        }))
        .on_action(cx.listener(|_, _: &FocusNext, window, cx| window.focus_next(cx)))
        .on_action(cx.listener(|_, _: &FocusPrev, window, cx| window.focus_prev(cx)))
        .on_key_down(cx.listener(|this, ev: &KeyDownEvent, _, cx| this.type_search(ev, cx)))
    }

    /// Only bound in the console's key context.
    pub(super) fn console_actions(el: Stateful<Div>, cx: &mut Context<Self>) -> Stateful<Div> {
        el.on_action(cx.listener(|this, _: &Submit, window, cx| run::submit(this, window, cx)))
            .on_action(cx.listener(|this, _: &SelectNext, _, cx| run::step(this, 1, cx)))
            .on_action(cx.listener(|this, _: &SelectPrev, _, cx| run::step(this, -1, cx)))
    }

    /// While search is open, plain typing edits the query; shortcuts
    /// with cmd or ctrl still reach their bindings. In the console any
    /// typed character opens the command bar, so typing is enough.
    fn type_search(&mut self, ev: &KeyDownEvent, cx: &mut Context<Self>) {
        let m = &ev.keystroke.modifiers;
        if m.platform || m.control {
            return;
        }
        let key = ev.keystroke.key.as_str();
        let typed = ev.keystroke.key_char.clone();
        self.state.update(cx, |s, cx| {
            let printable = typed.as_ref().filter(|c| !c.chars().any(char::is_control));
            if s.search.is_none() && s.design == Some(DesignKind::Console) && printable.is_some() {
                s.search = Some(String::new());
            }
            let Some(query) = s.search.as_mut() else {
                return;
            };
            match (key, printable) {
                ("backspace", _) => drop(query.pop()),
                (_, Some(c)) => query.push_str(c),
                _ => return,
            }
            s.cursor = 0;
            cx.notify();
        });
    }
}
