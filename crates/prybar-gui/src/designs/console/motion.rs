//! Console motion: crisp and snappy. Panes slide in over 110 ms, the
//! caret blinks, each keystroke flashes the prompt, and the highlight
//! springs between rows without overshoot.

use std::time::Duration;

use gpui::{
    ease_out_quint, prelude::*, px, Animation, AnimationExt as _, AnyElement, Div, ElementId,
    SpringAnimation, SpringConfig,
};

/// A pane arriving: 16 px from the right and fading in.
pub fn pane(el: Div, id: impl Into<ElementId>) -> AnyElement {
    el.with_animation(
        id,
        Animation::new(Duration::from_millis(110)).with_easing(ease_out_quint()),
        |el, t| el.opacity(t).ml(px(16. * (1. - t))),
    )
    .into_any_element()
}

/// A hard on/off blink, like a terminal caret.
pub fn caret(el: Div) -> AnyElement {
    el.with_animation(
        "cs-caret",
        Animation::new(Duration::from_millis(1000)).repeat(),
        |el, t| el.opacity(if t < 0.55 { 1. } else { 0. }),
    )
    .into_any_element()
}

/// Typing feedback: the id follows the query, so each key replays it.
pub fn flash(el: Div, id: impl Into<ElementId>) -> AnyElement {
    el.with_animation(id, Animation::new(Duration::from_millis(80)), |el, t| {
        el.opacity(0.35 + 0.65 * t)
    })
    .into_any_element()
}

/// Critically damped: the highlight snaps to `top` px and stops dead.
pub fn slide_to(el: Div, id: impl Into<ElementId>, top: f32) -> AnyElement {
    el.with_spring(
        id,
        SpringAnimation::new(SpringConfig::new(900., 60., 1.)).to(top),
        |el, v| el.top(px(v)),
    )
    .into_any_element()
}
