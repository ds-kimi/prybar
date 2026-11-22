//! Canvas motion: springy and playful but quick. Lists enter staggered,
//! the idle drop zone breathes, and a finished run bursts.

use std::time::Duration;

use gpui::{
    bounce, ease_in_out, ease_out_quint, prelude::*, px, Animation, AnimationExt as _, AnyElement,
    Div, ElementId, SpringAnimation, SpringConfig,
};

/// One item of a list entering: rises 10 px and fades in, `ix` items
/// after the first so the list ripples in.
pub fn enter(el: Div, id: impl Into<ElementId>, ix: usize) -> AnyElement {
    let delay = ix.min(12) as f32 * 50.;
    let total = 260. + delay;
    el.with_animation(
        id,
        Animation::new(Duration::from_millis(total as u64)),
        move |el, t| {
            let t = ease_out_quint()(((t * total - delay) / 260.).clamp(0., 1.));
            el.opacity(t).mt(px(10. * (1. - t)))
        },
    )
    .into_any_element()
}

/// The drop zone's icon bobbing while nothing is open.
pub fn idle(el: Div) -> AnyElement {
    el.with_animation(
        "cv-idle",
        Animation::new(Duration::from_millis(1800))
            .repeat()
            .with_easing(bounce(ease_in_out)),
        |el, t| el.mt(px(-8. * t)).mb(px(8. * t)),
    )
    .into_any_element()
}

/// Under-damped on purpose: it overshoots a little and settles.
/// A fill or highlight springing to `value` px wide (`width`) or from left.
pub fn spring_to(el: Div, id: impl Into<ElementId>, value: f32, width: bool) -> AnyElement {
    el.with_spring(
        id,
        SpringAnimation::new(SpringConfig::new(300., 18., 1.)).to(value),
        move |el, v| {
            if width {
                el.w(px(v.max(0.)))
            } else {
                el.left(px(v))
            }
        },
    )
    .into_any_element()
}

/// A ring growing and fading once; the id changes per run so it replays.
pub fn burst(el: Div, id: impl Into<ElementId>) -> AnyElement {
    el.with_animation(
        id,
        Animation::new(Duration::from_millis(700)).with_easing(ease_out_quint()),
        |el, t| el.size(px(24. + 96. * t)).opacity(1. - t),
    )
    .into_any_element()
}
