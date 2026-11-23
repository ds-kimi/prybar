//! One drawn line: plain runs, the selected run, the composing run, and
//! the caret; plus the probe that ties the field to the platform.

use std::cell::Cell;
use std::ops::Range;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    canvas, div, prelude::*, px, Animation, AnimationExt as _, AnyElement, Bounds, Canvas, Context,
    Div, ElementInputHandler, Entity, FocusHandle, Pixels, Rgba, Window,
};

use super::super::buffer::Op;
use super::super::step::Dir;
use super::super::{TextInput, LINE};
use crate::theme::prybar::Tokens;

/// What one row needs to know about the field around it.
pub struct Marks {
    pub sel: Range<usize>,
    pub marked: Option<Range<usize>>,
    pub head: Option<usize>,
}

/// Cuts the line where the selection, the composition and the caret
/// fall, so each run is drawn once and the caret sits between two.
pub fn row(text: &str, line: Range<usize>, m: &Marks, t: Tokens) -> Div {
    let clamp = |x: usize| x.clamp(line.start, line.end);
    let (sel, comp) = (m.sel.clone(), m.marked.clone().unwrap_or(0..0));
    let mut cuts = vec![line.start, line.end, clamp(sel.start), clamp(sel.end)];
    cuts.extend([clamp(comp.start), clamp(comp.end)]);
    let head = m.head.filter(|h| (line.start..=line.end).contains(h));
    cuts.extend(head);
    cuts.sort_unstable();
    cuts.dedup();
    let tint = Rgba { a: 0.32, ..t.rust };
    let mut el = div().h(LINE).flex().items_center().whitespace_nowrap();
    for w in cuts.windows(2) {
        if head == Some(w[0]) {
            el = el.child(caret(t));
        }
        let inside = |r: &Range<usize>| r.start <= w[0] && w[1] <= r.end && !r.is_empty();
        let run = div()
            .when(inside(&sel), |d| d.bg(tint))
            .when(inside(&comp), |d| d.underline())
            .child(text[w[0]..w[1]].to_owned());
        el = el.child(run);
    }
    if head == Some(line.end) {
        el = el.child(caret(t));
    }
    el
}

/// A rust bar that takes no width, so text never shifts around it; it
/// blinks slowly so a still caret is easy to find.
pub fn caret(t: Tokens) -> AnyElement {
    let bar = div()
        .absolute()
        .left(px(-0.75))
        .top(px(2.))
        .w(px(1.5))
        .h(px(16.))
        .bg(t.rust);
    div()
        .relative()
        .w(px(0.))
        .h(LINE)
        .child(bar)
        .with_animation(
            "caret",
            Animation::new(Duration::from_millis(1000)).repeat(),
            |el, x| el.opacity(if x < 0.6 { 1. } else { 0.2 }),
        )
        .into_any_element()
}

/// Fills the text area: it records where the text sits for the mouse,
/// and hands the platform this field as the place typed text goes.
pub fn probe(
    bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    focus: FocusHandle,
    field: Entity<TextInput>,
) -> Canvas<()> {
    canvas(
        move |b, window, _| {
            // A new width rewraps the text, which the frame already
            // drawn did not know; draw once more with it.
            let old = bounds.replace(Some(b)).map(|o| o.size.width);
            if old != Some(b.size.width) {
                window.on_next_frame(|window, _| window.refresh());
            }
        },
        move |b, _, window, cx| {
            window.handle_input(&focus, ElementInputHandler::new(b, field), cx);
        },
    )
}

impl TextInput {
    /// Up and down in a multi-line field follow the drawn rows, which
    /// only the view knows; true when the move was made here.
    pub(in super::super) fn by_row(
        &mut self,
        op: &Op,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Op::Move(d @ (Dir::Up | Dir::Down), select) = op else {
            return false;
        };
        if !self.spec.multiline {
            return false;
        }
        self.vertical(*d == Dir::Up, *select, window);
        cx.notify();
        true
    }
}
