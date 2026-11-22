//! Keys typed into a field, clipboard included.

use gpui::{ClipboardItem, Context, KeyDownEvent, Keystroke, Window};

use super::buffer::Op;
use super::step::key_op;
use super::{InputEvent, TextInput};

impl TextInput {
    /// Keys the field uses stop here; the rest (escape, shortcuts) go
    /// on to the app.
    pub(super) fn on_key(&mut self, ev: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let k = &ev.keystroke;
        if self.clipboard(k, cx) {
            cx.stop_propagation();
            return;
        }
        if k.key == "enter" && !self.spec.multiline {
            cx.stop_propagation();
            cx.emit(InputEvent::Submit);
            return;
        }
        let Some(op) = key_op(k, self.spec.multiline) else {
            return;
        };
        cx.stop_propagation();
        self.edit(op, cx);
    }

    fn clipboard(&mut self, k: &Keystroke, cx: &mut Context<Self>) -> bool {
        let m = &k.modifiers;
        let cmd = if cfg!(target_os = "macos") {
            m.platform
        } else {
            m.control
        };
        if !cmd {
            return false;
        }
        let picked = self.buf.text[self.buf.range()].to_owned();
        match k.key.as_str() {
            "c" | "x" if !picked.is_empty() => {
                cx.write_to_clipboard(ClipboardItem::new_string(picked));
                if k.key == "x" {
                    self.edit(Op::Back, cx);
                }
            }
            "c" | "x" => {}
            "v" => {
                let pasted = cx.read_from_clipboard().and_then(|i| i.text());
                let Some(text) = pasted else { return true };
                self.edit(Op::Insert(self.fit(text)), cx);
            }
            _ => return false,
        }
        true
    }

    /// One-line fields take pasted line breaks as spaces.
    fn fit(&self, text: String) -> String {
        if self.spec.multiline {
            return text.replace("\r\n", "\n");
        }
        text.replace("\r\n", " ").replace(['\r', '\n'], " ")
    }

    pub(super) fn edit(&mut self, op: Op, cx: &mut Context<Self>) {
        if self.buf.apply(op) {
            self.touched = true;
            cx.emit(InputEvent::Changed);
        }
        cx.notify();
    }
}
