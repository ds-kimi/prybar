// limits: allow fns -- EntityInputHandler needs all its methods in one impl
//! Text from the platform: plain typing, dead-key accents, the emoji
//! picker and IME composition all arrive here. The platform counts in
//! UTF-16; the buffer counts in bytes, so every range is converted.

use std::ops::Range;

use gpui::{
    point, px, Bounds, Context, EntityInputHandler, Pixels, Point, ShapedLine, TextRun,
    UTF16Selection, Window,
};

use super::buffer::Op;
use super::{TextInput, FONT, LINE};

/// UTF-16 units before byte `at`.
pub(super) fn to16(text: &str, at: usize) -> usize {
    text.char_indices()
        .take_while(|(i, _)| *i < at)
        .map(|(_, c)| c.len_utf16())
        .sum()
}

/// The byte offset of UTF-16 unit `u`, never inside a char.
pub(super) fn from16(text: &str, u: usize) -> usize {
    let mut n = 0;
    for (i, c) in text.char_indices() {
        if n >= u {
            return i;
        }
        n += c.len_utf16();
    }
    text.len()
}

impl TextInput {
    /// Shapes one line the way the field draws it, for hit tests and
    /// caret positions.
    pub(super) fn shape(&self, line: &str, window: &Window) -> ShapedLine {
        let style = window.text_style();
        let run = TextRun {
            len: line.len(),
            font: style.font(),
            color: style.color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        window
            .text_system()
            .shape_line(line.to_owned().into(), FONT, &[run], None)
    }

    /// A one-line field slides its text so the caret stays in view, and
    /// slides back when the text gets shorter.
    pub(super) fn follow_caret(&mut self, window: &Window) {
        let Some(b) = self.bounds.get().filter(|_| !self.spec.multiline) else {
            self.scroll = px(0.);
            return;
        };
        let shaped = self.shape(&self.buf.text, window);
        let x = shaped.x_for_index(self.buf.head);
        let w = (b.size.width - px(2.)).max(px(0.));
        if x - self.scroll > w {
            self.scroll = x - w;
        } else if x < self.scroll {
            self.scroll = x;
        }
        self.scroll = self.scroll.min((shaped.width - w).max(px(0.)));
    }

    fn bytes(&self, r: &Range<usize>) -> Range<usize> {
        let t = &self.buf.text;
        let (a, b) = (from16(t, r.start), from16(t, r.end));
        a.min(b)..a.max(b)
    }

    /// Writes `text` over the platform's range, or the composition, or
    /// the selection; returns where it landed.
    fn put(&mut self, r: Option<Range<usize>>, text: &str, cx: &mut Context<Self>) -> usize {
        let r = r
            .map(|r| self.bytes(&r))
            .or(self.marked.take())
            .unwrap_or_else(|| self.buf.range());
        self.buf.select_to(r.start, false);
        self.buf.select_to(r.end, true);
        // Keys like tab or escape can reach here as control chars; they
        // are not text.
        let text: String = text
            .chars()
            .filter(|c| *c == '\n' || !c.is_control())
            .collect();
        let text = self.fit(text);
        self.edit(Op::Insert(text), cx);
        r.start
    }
}

impl EntityInputHandler for TextInput {
    fn text_for_range(
        &mut self,
        r: Range<usize>,
        actual: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let r = self.bytes(&r);
        let t = &self.buf.text;
        actual.replace(to16(t, r.start)..to16(t, r.end));
        Some(t[r].to_owned())
    }

    fn selected_text_range(
        &mut self,
        _: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let (t, r) = (&self.buf.text, self.buf.range());
        Some(UTF16Selection {
            range: to16(t, r.start)..to16(t, r.end),
            reversed: self.buf.head < self.buf.anchor,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        let t = &self.buf.text;
        self.marked
            .as_ref()
            .map(|r| to16(t, r.start)..to16(t, r.end))
    }

    fn unmark_text(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.marked = None;
        cx.notify();
    }

    fn replace_text_in_range(
        &mut self,
        r: Option<Range<usize>>,
        text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.put(r, text, cx);
    }

    /// Composition: the text stays marked (drawn underlined) until the
    /// input method commits it; the platform picks the caret inside it.
    fn replace_and_mark_text_in_range(
        &mut self,
        r: Option<Range<usize>>,
        text: &str,
        pick: Option<Range<usize>>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let start = self.put(r, text, cx);
        let end = self.buf.head;
        self.marked = (end > start).then_some(start..end);
        if let Some(p) = pick {
            let base = to16(&self.buf.text, start);
            let at = |u: usize| from16(&self.buf.text, base + u).clamp(start, end);
            let (a, h) = (at(p.start), at(p.end));
            self.buf.select_to(a, false);
            self.buf.select_to(h, true);
        }
    }

    /// Where the IME draws its candidate window: under the range.
    fn bounds_for_range(
        &mut self,
        r: Range<usize>,
        b: Bounds<Pixels>,
        window: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let r = self.bytes(&r);
        let rows = self.rows(window);
        let row = super::lines::row_of(&rows, r.start);
        let line = rows.get(row).cloned().unwrap_or(0..0);
        let shaped = self.shape(&self.buf.text[line.clone()], window);
        let at = |i: usize| i.clamp(line.start, line.end) - line.start;
        let x = |i: usize| b.left() + shaped.x_for_index(at(i)) - self.scroll;
        let top = b.top() + LINE * row as f32 - self.vscroll;
        Some(Bounds::from_corners(
            point(x(r.start), top),
            point(x(r.end), top + LINE),
        ))
    }

    fn character_index_for_point(
        &mut self,
        p: Point<Pixels>,
        window: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<usize> {
        let at = self.index_at(p, window);
        Some(to16(&self.buf.text, at))
    }

    fn set_selected_text_range(&mut self, r: Range<usize>, _: &mut Window, cx: &mut Context<Self>) {
        let r = self.bytes(&r);
        self.buf.select_to(r.start, false);
        self.buf.select_to(r.end, true);
        cx.notify();
    }

    fn text_length_utf16(&mut self, _: &mut Window, _: &mut Context<Self>) -> Option<usize> {
        Some(to16(&self.buf.text, self.buf.text.len()))
    }
}
