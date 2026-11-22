//! A real text field: caret, selection by mouse and shift-arrows, copy,
//! cut, paste and undo, one line or several. Editing keys arrive as key
//! events; text itself comes through the platform input handler, so
//! accents, the emoji picker and IME composition work.

mod buffer;
mod edit;
mod ime;
mod keys;
mod mouse;
mod step;
#[cfg(test)]
mod tests;
mod text;
mod view;

use std::{cell::Cell, ops::Range, rc::Rc};

use gpui::{px, Bounds, Context, EventEmitter, FocusHandle, Pixels};

use buffer::Buffer;
pub use step::any;

/// Height of one text line, also the step of clicks between lines.
const LINE: Pixels = px(20.);
/// Text size of fields; the mouse hit test shapes text at this size.
const FONT: Pixels = px(14.);

/// What a field tells its owner.
pub enum InputEvent {
    Changed,
    /// Enter in a one-line field.
    Submit,
}

/// How a field starts: hint, kind and the rule its text must follow.
pub struct Spec {
    pub placeholder: &'static str,
    pub multiline: bool,
    pub text: String,
    /// A plain message when the text is not usable, `None` when it is.
    pub check: fn(&str) -> Option<&'static str>,
}

pub struct TextInput {
    buf: Buffer,
    focus: FocusHandle,
    spec: Spec,
    /// Where the text was last drawn; mouse positions are read against it.
    bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    dragging: bool,
    /// Text the input method is still composing, drawn underlined.
    marked: Option<Range<usize>>,
    /// How far a one-line field is slid left to keep the caret in view.
    scroll: Pixels,
    /// Errors wait for the first edit, so a fresh form is not all red.
    touched: bool,
}

impl EventEmitter<InputEvent> for TextInput {}

impl TextInput {
    pub fn new(spec: Spec, cx: &mut Context<Self>) -> Self {
        let mut buf = Buffer::default();
        buf.set(&spec.text);
        TextInput {
            buf,
            focus: cx.focus_handle().tab_index(0).tab_stop(true),
            spec,
            bounds: Rc::default(),
            dragging: false,
            marked: None,
            scroll: px(0.),
            touched: false,
        }
    }

    pub fn text(&self) -> &str {
        &self.buf.text
    }

    /// The app replacing the text, such as clearing search; not an edit.
    pub fn set_text(&mut self, text: &str, cx: &mut Context<Self>) {
        self.buf.set(text);
        self.marked = None;
        self.touched = false;
        cx.notify();
    }

    pub fn error(&self) -> Option<&'static str> {
        (self.spec.check)(&self.buf.text)
    }
}
