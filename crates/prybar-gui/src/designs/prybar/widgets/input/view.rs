//! Drawing a field: the box with its hover, focus and error states, the
//! text or the hint, and the error line under it.

use gpui::{
    canvas, div, prelude::*, px, App, Context, Div, FocusHandle, Focusable, MouseButton, Stateful,
    Window,
};

use super::text::{caret, row};
use super::{TextInput, FONT, LINE};
use crate::theme::prybar::{tokens, Light, Tokens};

impl Render for TextInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(cx.try_global::<Light>().is_some_and(|l| l.0));
        let focused = self.focus.is_focused(window);
        let error = self.error().filter(|_| self.touched);
        let bounds = self.bounds.clone();
        let probe = canvas(move |b, _, _| bounds.set(Some(b)), |_, _, _, _| {})
            .absolute()
            .size_full();
        let text = div()
            .relative()
            .flex_1()
            .min_w_0()
            .child(probe)
            .child(self.lines(focused, t));
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
            .key_context("TextInput")
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
                el.min_h(px(96.)).overflow_y_scroll()
            })
            .hover(move |s| s.border_color(t.muted))
            .focus(move |s| s.border_color(t.rust))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_down))
            .on_mouse_move(cx.listener(Self::on_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_up))
            .on_key_down(cx.listener(Self::on_key))
    }

    /// One row per line; an empty field shows its hint after the caret.
    fn lines(&self, focused: bool, t: Tokens) -> Div {
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
        let head = focused.then_some(self.buf.head);
        let mut start = 0;
        let mut rows = Vec::new();
        for line in text.split('\n') {
            let end = start + line.len();
            rows.push(row(text, start..end, self.buf.range(), head, t));
            start = end + 1;
        }
        div().flex().flex_col().children(rows)
    }
}
