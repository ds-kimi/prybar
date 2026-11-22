//! Later-work regions, shared by every design.

use std::time::Duration;

use gpui::{div, prelude::*, pulsating_between, Animation, AnimationExt as _, AnyElement};

use crate::state::Placeholder;
use crate::theme::Palette;

/// A breathing dashed region: clearly not built yet, never a dead box.
pub fn region(ph: Placeholder, p: Palette) -> AnyElement {
    div()
        .id("placeholder")
        .debug_selector(|| "placeholder".into())
        .flex_1()
        .min_h_64()
        .flex()
        .items_center()
        .justify_center()
        .rounded_xl()
        .border_2()
        .border_dashed()
        .border_color(p.border)
        .text_color(p.muted)
        .child(ph.label())
        .with_animation(
            "placeholder-breath",
            Animation::new(Duration::from_millis(2200))
                .repeat()
                .with_easing(pulsating_between(0.5, 1.)),
            |el, t| el.opacity(t),
        )
        .into_any_element()
}
