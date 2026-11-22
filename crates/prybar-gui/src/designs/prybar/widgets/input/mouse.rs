//! Clicks and drags: place the caret, extend with shift, select by
//! dragging, select all on a double click.

use gpui::{Context, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, TextRun, Window};

use super::{TextInput, FONT, LINE};

impl TextInput {
    /// The text offset under a window position, by shaping the line
    /// under it the way it is drawn.
    fn index_at(&self, p: Point<Pixels>, window: &Window) -> usize {
        let text = &self.buf.text;
        let Some(b) = self.bounds.get() else {
            return text.len();
        };
        let lines: Vec<&str> = text.split('\n').collect();
        let row = ((p.y - b.origin.y) / LINE).floor().max(0.) as usize;
        let row = row.min(lines.len().saturating_sub(1));
        let start: usize = lines[..row].iter().map(|l| l.len() + 1).sum();
        let line = lines.get(row).copied().unwrap_or("");
        if line.is_empty() {
            return start;
        }
        let style = window.text_style();
        let run = TextRun {
            len: line.len(),
            font: style.font(),
            color: style.color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let shaped = window
            .text_system()
            .shape_line(line.to_owned().into(), FONT, &[run], None);
        start + shaped.closest_index_for_x(p.x - b.origin.x)
    }

    pub(super) fn on_down(
        &mut self,
        ev: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.focus.focus(window, cx);
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
