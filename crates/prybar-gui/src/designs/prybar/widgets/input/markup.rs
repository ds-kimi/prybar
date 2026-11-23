//! What a field that takes markup offers its owner: formatting keys
//! and putting tags around the selection.

use gpui::{Context, Keystroke};

use super::buffer::Op;
use super::{InputEvent, TextInput};

/// The keys word processors use for bold, italic, underline and link.
const KEYS: [&str; 4] = ["b", "i", "u", "k"];

impl TextInput {
    /// Mod plus a formatting key, in a field that takes markup, goes to
    /// the owner; elsewhere these keys stay app shortcuts.
    pub(super) fn shortcut(&mut self, k: &Keystroke, cx: &mut Context<Self>) -> bool {
        let m = &k.modifiers;
        let cmd = if cfg!(target_os = "macos") {
            m.platform
        } else {
            m.control
        };
        let key = KEYS.into_iter().find(|key| *key == k.key);
        match key {
            Some(key) if self.spec.markup && cmd && !m.shift && !m.alt => {
                cx.emit(InputEvent::Format(key));
                true
            }
            _ => false,
        }
    }

    /// Puts `open` and `close` around the selection and keeps what was
    /// selected selected; with nothing selected the caret lands between
    /// them, ready to type. One undo step takes it all back.
    pub fn surround(&mut self, open: &str, close: &str, cx: &mut Context<Self>) {
        self.marked = None;
        let r = self.buf.range();
        let inner = self.buf.text[r.clone()].to_owned();
        self.edit(Op::Insert(format!("{open}{inner}{close}")), cx);
        let start = r.start + open.len();
        self.buf.select_to(start, false);
        self.buf.select_to(start + inner.len(), true);
    }

    /// The selected text, for tags that take it as their value.
    pub fn selected(&self) -> &str {
        self.buf.text.get(self.buf.range()).unwrap_or("")
    }
}
