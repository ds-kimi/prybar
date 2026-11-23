//! Drawing a field: the box with its hover, focus and error states, the
//! text or the hint, and the error line under it.

use std::ops::Range;

use gpui::{
    div, prelude::*, App, Context, Div, FocusHandle, Focusable, MouseButton, Stateful, Window,
};

use super::lines::row_of;
use super::lines::text::{caret, probe, row, Marks};
use super::{InputEvent, TextInput, FONT, LINE};
use crate::actions::FormatLink;
use crate::theme::prybar::{tokens, Light, Tokens};

impl Render for TextInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(cx.try_global::<Light>().is_some_and(|l| l.0));
        let focused = self.focus.is_focused(window);
        let error = self.error().filter(|_| self.touched);
        self.follow_caret(window);
        let rows = self.rows(window);
        let shown = self.spec.multiline.then(|| self.follow_rows(&rows));
        let probe = probe(self.bounds.clone(), self.focus.clone(), cx.entity());
        let lines = self.lines(focused, &rows, t).relative();
        let lines = lines.left(-self.scroll).top(-self.vscroll);
        let text = div()
            .relative()
            .flex_1()
            .min_w_0()
            .when_some(shown, |el, h| el.h(h).overflow_hidden())
            .child(probe.absolute().size_full())
            .child(lines);
        let field = self.frame(error.is_some(), t, cx).child(text);
        let message = error.map(|e| div().text_xs().text_color(t.danger).child(e));
        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(field)
            .children(message)
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl TextInput {
    /// The box: border turns muted on hover, rust with focus, red on error.
    fn frame(&self, error: bool, t: Tokens, cx: &mut Context<Self>) -> Stateful<Div> {
        let edge = if error { t.danger } else { t.border };
        div()
            .id("input")
            .track_focus(&self.focus)
            .key_context(["TextInput", "TextInput Markup"][self.spec.markup as usize])
            .on_action(cx.listener(|_, _: &FormatLink, _, cx| cx.emit(InputEvent::Format("k"))))
            .px_2()
            .py_1p5()
            .flex()
            .rounded_md()
            .border_1()
            .border_color(edge)
            .bg(t.bg)
            .overflow_hidden()
            .text_size(FONT)
            .line_height(LINE)
            .cursor_text()
            .when(self.spec.multiline, |el| {
                el.on_scroll(cx.listener(Self::on_wheel))
            })
            .hover(move |s| s.border_color(t.muted))
            .focus(move |s| s.border_color(t.rust))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_down))
            .on_mouse_move(cx.listener(Self::on_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_up))
            .on_key_down(cx.listener(Self::on_key))
    }

    /// One element per drawn row; an empty field shows its hint after
    /// the caret, which is drawn on one row only even at a wrap point.
    fn lines(&self, focused: bool, rows: &[Range<usize>], t: Tokens) -> Div {
        let text = &self.buf.text;
        if text.is_empty() {
            let hint = div().text_color(t.muted).child(self.spec.placeholder);
            return div()
                .h(LINE)
                .flex()
                .items_center()
                .when(focused, |el| el.child(caret(t)))
                .child(hint);
        }
        let at = row_of(rows, self.buf.head);
        let drawn = rows.iter().enumerate().map(|(i, r)| {
            let marks = Marks {
                sel: self.buf.range(),
                marked: self.marked.clone(),
                head: (focused && i == at).then_some(self.buf.head),
            };
            row(text, r.clone(), &marks, t)
        });
        div().flex().flex_col().children(drawn)
    }
}
