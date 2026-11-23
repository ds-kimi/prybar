//! Motion: short and eased, never decorative. GPUI's animations already
//! honor the system's reduce-motion setting.

use std::time::Duration;

use gpui::{
    ease_out_quint, prelude::*, px, relative, Animation, AnimationExt as _, AnyElement, Div,
    ElementId,
};

use crate::state::toast::{ENTER, LEAVE};

/// Taller than a two-line toast: the room animates up to this, then the
/// cap is lifted so longer text still shows whole.
const ROOM: f32 = 72.;
/// How far a toast travels while sliding.
const SLIDE: f32 = 32.;

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

/// Entering, the room opens first and the toast slides in from the right;
/// leaving, it slides out and then the room closes, so the rest glide.
pub fn toast(room: Div, id: usize, leaving: bool) -> AnyElement {
    if leaving {
        let out = |el: Div, k: f32| {
            let (slide, shut) = ((k / 0.57).min(1.), ((k - 0.57) / 0.43).clamp(0., 1.));
            let el = el.max_h(px(ROOM * (1. - shut))).opacity(1. - slide);
            el.mr(px(-SLIDE * slide * slide))
        };
        return room
            .with_animation(("pb-toast-out", id), Animation::new(LEAVE), out)
            .into_any_element();
    }
    let anim = Animation::new(ENTER).with_easing(ease_out_quint());
    room.with_animation(("pb-toast-in", id), anim, |el, k| {
        let (open, slide) = ((k / 0.3).min(1.), ((k - 0.3) / 0.7).clamp(0., 1.));
        let el = el.opacity(slide).mr(px(-SLIDE * (1. - slide)));
        if k < 1. {
            el.max_h(px(ROOM * open))
        } else {
            el
        }
    })
    .into_any_element()
}
