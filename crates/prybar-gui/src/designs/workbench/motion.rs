//! Workbench motion: quick and subtle, 120-180 ms ease-out, springs for
//! highlights that follow the selection.

use std::time::Duration;

use gpui::{
    ease_out_quint, prelude::*, px, Animation, AnimationExt as _, AnyElement, Div, ElementId,
    SpringAnimation, SpringConfig,
};

/// Stiff and well damped: arrives in about 150 ms with no visible wobble.
fn spring() -> SpringConfig {
    SpringConfig::new(420., 38., 1.)
}

/// A highlight that slides to `offset` along x (`horizontal`) or y.
pub fn slide_to(el: Div, id: impl Into<ElementId>, offset: f32, horizontal: bool) -> AnyElement {
    el.with_spring(
        id,
        SpringAnimation::new(spring()).to(offset),
        move |el, v| {
            if horizontal {
                el.left(px(v))
            } else {
                el.top(px(v))
            }
        },
    )
    .into_any_element()
}

/// Content entering: fades in and settles 6 px from the right. The id
/// must change per screen so the motion replays on each navigation.
pub fn enter(el: Div, id: impl Into<ElementId>) -> AnyElement {
    el.with_animation(
        id,
        Animation::new(Duration::from_millis(160)).with_easing(ease_out_quint()),
        |el, t| el.opacity(t).ml(px(6. * (1. - t))),
    )
    .into_any_element()
}

/// A light band sweeping across a running progress bar.
pub fn shimmer(el: Div, width: f32) -> AnyElement {
    el.with_animation(
        "status-shimmer",
        Animation::new(Duration::from_millis(1100)).repeat(),
        move |el, t| el.left(px(-40. + (width + 40.) * t)),
    )
    .into_any_element()
}
