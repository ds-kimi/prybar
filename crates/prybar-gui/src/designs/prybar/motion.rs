//! Motion: short and eased, never decorative. GPUI's animations already
//! honor the system's reduce-motion setting.

use std::time::Duration;

use gpui::{
    ease_out_quint, prelude::*, px, relative, Animation, AnimationExt as _, AnyElement, Div,
    ElementId,
};

/// Content arriving: fades in while settling 4 px up. A new id per
/// screen replays it on every navigation.
pub fn enter(el: Div, id: impl Into<ElementId>) -> AnyElement {
    el.with_animation(
        id,
        Animation::new(Duration::from_millis(150)).with_easing(ease_out_quint()),
        |el, t| el.opacity(t).mt(px(4. * (1. - t))),
    )
    .into_any_element()
}

/// The current nav item's underline drawing out from its center.
pub fn underline(el: Div, id: impl Into<ElementId>) -> AnyElement {
    el.with_animation(
        id,
        Animation::new(Duration::from_millis(180)).with_easing(ease_out_quint()),
        |el, t| el.w(relative(t)).left(relative((1. - t) / 2.)),
    )
    .into_any_element()
}

/// A short rust band crossing the bar while work runs; its length
/// is unknown, so this says "busy" rather than faking a percentage.
pub fn sweep(el: Div) -> AnyElement {
    el.with_animation(
        "pb-sweep",
        Animation::new(Duration::from_millis(1200)).repeat(),
        |el, t| el.left(relative(-0.2 + 1.2 * t)),
    )
    .into_any_element()
}
