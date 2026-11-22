//! Keyboard behavior of the select, the way native pop-up buttons work.

use gpui::{Context, KeyDownEvent, Window};

use super::Select;

impl Select {
    pub(super) fn key(&mut self, e: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let n = self.choices.len();
        if n == 0 {
            return;
        }
        let key = e.keystroke.key.as_str();
        match (key, self.open) {
            ("escape", true) => self.open = false,
            ("enter" | "space", true) => return self.pick(self.hi, cx),
            ("enter" | "space" | "down" | "up", false) => self.toggle(cx),
            ("down", true) => self.hi = (self.hi + 1).min(n - 1),
            ("up", true) => self.hi = self.hi.saturating_sub(1),
            ("home", true) => self.hi = 0,
            ("end", true) => self.hi = n - 1,
            _ => return self.jump(key, cx),
        }
        cx.stop_propagation();
        cx.notify();
    }

    /// Typing a letter moves to the next choice starting with it, wrapping,
    /// so repeated presses cycle through the matches.
    fn jump(&mut self, key: &str, cx: &mut Context<Self>) {
        let mut chars = key.chars();
        let (Some(c), None) = (chars.next(), chars.next()) else {
            return;
        };
        let n = self.choices.len();
        let from = if self.open { self.hi } else { self.current };
        let hit = (1..=n).map(|k| (from + k) % n).find(|&i| {
            let first = self.choices[i].label.chars().next();
            first.is_some_and(|f| f.eq_ignore_ascii_case(&c))
        });
        if let Some(i) = hit {
            if self.open {
                self.hi = i;
                cx.notify();
            } else {
                self.pick(i, cx);
            }
            cx.stop_propagation();
        }
    }

    pub(super) fn toggle(&mut self, cx: &mut Context<Self>) {
        self.open = !self.open && !self.choices.is_empty();
        self.hi = self.current;
        cx.notify();
    }
}
