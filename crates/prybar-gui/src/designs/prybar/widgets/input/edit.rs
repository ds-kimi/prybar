//! Buffer moves that need more than one line: arrows over a selection,
//! undo, and setting the text from outside.

use super::buffer::Buffer;
use super::step::{step, Dir};

impl Buffer {
    /// Without shift, left or right on a selection lands on its edge
    /// instead of stepping, like every native field.
    pub fn move_caret(&mut self, dir: Dir, select: bool) {
        let r = self.range();
        let at = match (dir, select, r.is_empty()) {
            (Dir::Left, false, false) => r.start,
            (Dir::Right, false, false) => r.end,
            _ => step(&self.text, self.head, dir),
        };
        self.select_to(at, select);
    }

    pub fn undo_last(&mut self) -> bool {
        let Some((text, head)) = self.undo.pop() else {
            return false;
        };
        self.text = text;
        self.select_to(head, false);
        true
    }

    /// Text set by the app, not typed: the caret goes to the end and
    /// there is nothing to undo back to.
    pub fn set(&mut self, text: &str) {
        self.text = text.to_owned();
        self.undo.clear();
        self.select_to(text.len(), false);
    }
}
