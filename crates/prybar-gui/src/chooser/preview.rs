//! Looping mini previews that hint at each design's layout and motion.

use std::time::Duration;

use gpui::{div, prelude::*, pulsating_between, px, rgb, Animation, AnimationExt as _, AnyElement};

use crate::state::DesignKind;
use crate::theme;

pub fn preview(kind: DesignKind) -> AnyElement {
    let p = theme::for_design(kind);
    let frame = div()
        .h_32()
        .w_full()
        .rounded_lg()
        .overflow_hidden()
        .bg(p.bg)
        .p_2()
        .flex()
        .gap_1();
    let inner = match kind {
        DesignKind::Workbench | DesignKind::Prybar => workbench(p.accent.into(), p.surface.into()),
        DesignKind::Canvas => canvas(p.accent.into(), p.surface.into()),
        DesignKind::Console => console(p.accent.into()),
    };
    frame.child(inner).into_any_element()
}

/// Three panes and a selection bar sliding down a tree.
fn workbench(accent: gpui::Hsla, surface: gpui::Hsla) -> AnyElement {
    let pane = |w: f32| div().h_full().w(px(w)).rounded_sm().bg(surface);
    let bar = div()
        .absolute()
        .left_0()
        .w_full()
        .h(px(8.))
        .rounded_sm()
        .bg(accent)
        .opacity(0.6);
    div()
        .size_full()
        .flex()
        .gap_1()
        .child(pane(60.).relative().child(bar.with_animation(
            "bench-sel",
            Animation::new(Duration::from_millis(2400)).repeat(),
            |el, t| el.top(px(4. + 90. * t)),
        )))
        .child(pane(130.))
        .child(pane(60.))
        .into_any_element()
}

/// Rounded step cards breathing in turn.
fn canvas(accent: gpui::Hsla, surface: gpui::Hsla) -> AnyElement {
    let step = |i: usize| {
        div()
            .size(px(64.))
            .rounded_xl()
            .bg(if i == 1 { accent } else { surface })
            .shadow_md()
            .with_animation(
                ("canvas-step", i),
                Animation::new(Duration::from_millis(1800))
                    .repeat()
                    .with_easing(pulsating_between(0.55, 1.)),
                |el, t| el.opacity(t),
            )
    };
    div()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .gap_3()
        .children((0..3).map(step))
        .into_any_element()
}

/// Prompt lines and a blinking cursor.
fn console(accent: gpui::Hsla) -> AnyElement {
    let line = |w: f32| div().h(px(6.)).w(px(w)).rounded_sm().bg(rgb(0x3a3f47));
    let cursor = div().h(px(10.)).w(px(6.)).bg(accent).with_animation(
        "console-cursor",
        Animation::new(Duration::from_millis(1000))
            .repeat()
            .with_easing(pulsating_between(0., 1.)),
        |el, t| el.opacity(t),
    );
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_2()
        .p_2()
        .children([line(160.), line(110.), line(190.)])
        .child(
            div()
                .flex()
                .gap_1()
                .items_center()
                .child(line(40.).bg(accent))
                .child(cursor),
        )
        .into_any_element()
}
