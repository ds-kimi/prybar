//! The rows of a field as drawn: up and down by row, and the vertical
//! scroll of a multi-line field that has more rows than it shows.

use std::ops::Range;

use gpui::{px, Context, Pixels, ScrollEvent, Window};

use super::super::step::{step, Dir};
use super::super::{TextInput, LINE};
use super::{edges, row_of, wrap_line, x_at};

impl TextInput {
    /// One row per line; a multi-line field also breaks long lines to
    /// its width as last drawn. Never empty.
    pub(in super::super) fn rows(&self, window: &Window) -> Vec<Range<usize>> {
        let text = &self.buf.text;
        let max = self.bounds.get().filter(|_| self.spec.multiline);
        let max = max.map(|b| f32::from(b.size.width) - 2.);
        let mut rows = Vec::new();
        let mut start = 0;
        for line in text.split('\n') {
            match max {
                Some(max) if !line.is_empty() => {
                    let xs = edges(&self.shape(line, window));
                    let wrapped = wrap_line(line, max, |i| x_at(&xs, i));
                    rows.extend(wrapped.into_iter().map(|r| r.start + start..r.end + start));
                }
                _ => rows.push(start..start + line.len()),
            }
            start += line.len() + 1;
        }
        rows
    }

    /// Up and down go by drawn row, keeping the caret's x; past the
    /// first or last row they go to the start or the end.
    pub(in super::super) fn vertical(&mut self, up: bool, select: bool, window: &Window) {
        let rows = self.rows(window);
        let r = row_of(&rows, self.buf.head);
        let target = if up { r.checked_sub(1) } else { Some(r + 1) };
        let (Some(from), Some(to)) = (rows.get(r).cloned(), target.and_then(|t| rows.get(t)))
        else {
            let at = if up { 0 } else { self.buf.text.len() };
            return self.buf.select_to(at, select);
        };
        let text = &self.buf.text;
        let x = self
            .shape(&text[from.clone()], window)
            .x_for_index(self.buf.head - from.start);
        let mut at = to.start + self.shape(&text[to.clone()], window).closest_index_for_x(x);
        // The end of a wrapped row is where the next starts; one char
        // back keeps the caret on the row it was sent to.
        let wrapped = rows.iter().any(|n| n.start == to.end);
        if at == to.end && at > to.start && wrapped {
            at = step(text, at, Dir::Left);
        }
        self.buf.select_to(at, select);
    }

    /// Shows between the spec's least and most rows, then scrolls; the
    /// caret's row is brought into view when it moved, so a wheel scroll
    /// is left alone until the next edit. Returns the shown height.
    pub(in super::super) fn follow_rows(&mut self, rows: &[Range<usize>]) -> Pixels {
        let (least, most) = self.spec.rows;
        let shown = LINE * rows.len().clamp(least, most.max(least)) as f32;
        let key = (self.buf.head, self.buf.text.len());
        if self.followed != key {
            self.followed = key;
            let top = LINE * row_of(rows, self.buf.head) as f32;
            if top < self.vscroll {
                self.vscroll = top;
            } else if top + LINE > self.vscroll + shown {
                self.vscroll = top + LINE - shown;
            }
        }
        self.vmax = (LINE * rows.len() as f32 - shown).max(px(0.));
        self.vscroll = self.vscroll.min(self.vmax).max(px(0.));
        shown
    }

    /// The wheel scrolls the field while it can, then the page.
    pub(in super::super) fn on_wheel(
        &mut self,
        ev: &ScrollEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let dy = ev.delta.pixel_delta(LINE).y;
        let next = (self.vscroll - dy).min(self.vmax).max(px(0.));
        if next != self.vscroll {
            self.vscroll = next;
            cx.stop_propagation();
            cx.notify();
        }
    }
}
