//! The text and selection of one field, apart from any drawing, so the
//! editing rules are plain to test.

use std::ops::Range;

use super::step::{step, Dir};

/// Undo keeps this many steps; older ones drop off.
const MAX_UNDO: usize = 100;

pub enum Op {
    Insert(String),
    Back,
    Forward,
    Move(Dir, bool),
    SelectAll,
    Undo,
}

#[derive(Default)]
pub struct Buffer {
    pub text: String,
    /// The fixed end of the selection; equals `head` when nothing is selected.
    pub anchor: usize,
    /// Where the caret is; moves with arrows and the mouse.
    pub head: usize,
    pub(super) undo: Vec<(String, usize)>,
}

impl Buffer {
    pub fn range(&self) -> Range<usize> {
        self.anchor.min(self.head)..self.anchor.max(self.head)
    }

    /// Moves the caret; with `select` the anchor stays so the selection
    /// grows. Offsets past the end or inside a char are pulled back.
    pub fn select_to(&mut self, at: usize, select: bool) {
        let mut at = at.min(self.text.len());
        while !self.text.is_char_boundary(at) {
            at -= 1;
        }
        self.head = at;
        if !select {
            self.anchor = at;
        }
    }

    /// Replaces the selection; true when the text changed.
    pub fn replace(&mut self, with: &str) -> bool {
        let r = self.range();
        if r.is_empty() && with.is_empty() {
            return false;
        }
        self.undo.push((self.text.clone(), self.head));
        if self.undo.len() > MAX_UNDO {
            self.undo.remove(0);
        }
        self.text.replace_range(r.clone(), with);
        self.select_to(r.start + with.len(), false);
        true
    }

    /// Applies one edit; true when the text changed.
    pub fn apply(&mut self, op: Op) -> bool {
        match op {
            Op::Insert(s) => self.replace(&s),
            Op::Back | Op::Forward if self.range().is_empty() => {
                let dir = if matches!(op, Op::Back) {
                    Dir::Left
                } else {
                    Dir::Right
                };
                self.select_to(step(&self.text, self.head, dir), true);
                self.replace("")
            }
            Op::Back | Op::Forward => self.replace(""),
            Op::Move(dir, select) => {
                self.move_caret(dir, select);
                false
            }
            Op::SelectAll => {
                self.anchor = 0;
                self.head = self.text.len();
                false
            }
            Op::Undo => self.undo_last(),
        }
    }
}
