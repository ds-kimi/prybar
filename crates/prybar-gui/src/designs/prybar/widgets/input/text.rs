//! One drawn line: plain runs, the selected run, and the caret.

use std::ops::Range;
use std::time::Duration;

use gpui::{div, prelude::*, px, Animation, AnimationExt as _, AnyElement, Div, Rgba};

use super::LINE;
use crate::theme::prybar::Tokens;

/// Cuts the line where the selection and the caret fall, so each run is
/// drawn once and the caret sits between two of them.
pub fn row(
    text: &str,
    line: Range<usize>,
    sel: Range<usize>,
    head: Option<usize>,
    t: Tokens,
) -> Div {
    let clamp = |x: usize| x.clamp(line.start, line.end);
    let mut cuts = vec![line.start, line.end, clamp(sel.start), clamp(sel.end)];
    let head = head.filter(|h| (line.start..=line.end).contains(h));
    cuts.extend(head);
    cuts.sort_unstable();
    cuts.dedup();
    let tint = Rgba { a: 0.32, ..t.rust };
    let mut el = div().h(LINE).flex().items_center().whitespace_nowrap();
    for w in cuts.windows(2) {
        if head == Some(w[0]) {
            el = el.child(caret(t));
        }
        let picked = sel.start <= w[0] && w[1] <= sel.end && !sel.is_empty();
        let run = div()
            .when(picked, |d| d.bg(tint))
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
