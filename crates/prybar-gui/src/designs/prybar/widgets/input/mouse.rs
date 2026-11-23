//! Clicks and drags: place the caret, extend with shift, select by
//! dragging, select all on a double click.

use gpui::{Context, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, Window};

use super::{TextInput, LINE};

impl TextInput {
    /// The text offset under a window position, by shaping the row
    /// under it the way it is drawn.
    pub(super) fn index_at(&self, p: Point<Pixels>, window: &Window) -> usize {
        let text = &self.buf.text;
        let Some(b) = self.bounds.get() else {
            return text.len();
        };
        let rows = self.rows(window);
        let row = ((p.y - b.origin.y + self.vscroll) / LINE).floor().max(0.) as usize;
        let Some(r) = rows.get(row).or(rows.last()) else {
            return text.len();
        };
        let shaped = self.shape(&text[r.clone()], window);
        r.start + shaped.closest_index_for_x(p.x - b.origin.x + self.scroll)
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
