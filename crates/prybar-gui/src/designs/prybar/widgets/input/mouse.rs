//! Clicks and drags: place the caret, extend with shift, select by
//! dragging, select all on a double click.

use gpui::{Context, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, Window};

use super::{TextInput, LINE};

impl TextInput {
    /// The text offset under a window position, by shaping the line
    /// under it the way it is drawn.
    pub(super) fn index_at(&self, p: Point<Pixels>, window: &Window) -> usize {
        let text = &self.buf.text;
        let Some(b) = self.bounds.get() else {
            return text.len();
        };
        let lines: Vec<&str> = text.split('\n').collect();
        let row = ((p.y - b.origin.y) / LINE).floor().max(0.) as usize;
        let row = row.min(lines.len().saturating_sub(1));
        let start: usize = lines[..row].iter().map(|l| l.len() + 1).sum();
        let line = lines.get(row).copied().unwrap_or("");
        let shaped = self.shape(line, window);
        start + shaped.closest_index_for_x(p.x - b.origin.x + self.scroll)
    }

    pub(super) fn on_down(
        &mut self,
        ev: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.focus.focus(window, cx);
        // A click ends a composition where it stands, so the caret lands
        // where the user clicked rather than inside the marked run.
        self.marked = None;
        if ev.click_count >= 2 {
            self.buf.anchor = 0;
            self.buf.head = self.buf.text.len();
        } else {
            let at = self.index_at(ev.position, window);
            self.buf.select_to(at, ev.modifiers.shift);
            self.dragging = true;
        }
        cx.notify();
    }

    pub(super) fn on_move(
        &mut self,
        ev: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.dragging && ev.dragging() {
            let at = self.index_at(ev.position, window);
            self.buf.select_to(at, true);
            cx.notify();
        }
    }

    pub(super) fn on_up(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut Context<Self>) {
        self.dragging = false;
    }
}
