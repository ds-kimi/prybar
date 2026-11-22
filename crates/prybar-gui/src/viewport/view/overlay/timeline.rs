//! Playback strip along the bottom while a sequence plays: play or
//! pause, speed, and a bar to scrub.

use std::cell::Cell;
use std::rc::Rc;

use gpui::{canvas, div, prelude::*, px, Bounds, Context, MouseButton, Pixels};

use super::chip;
use crate::theme::prybar::Tokens;
use crate::viewport::pose::SPEEDS;
use crate::viewport::view::Viewport;

pub fn timeline(v: &Viewport, t: &Tokens, cx: &mut Context<Viewport>) -> Option<impl IntoElement> {
    let clip = v.play.clip.as_ref()?;
    let p = &v.play;
    let play = if p.paused {
        "Play  Space"
    } else {
        "Pause  Space"
    };
    let speed = format!("{}×", p.rate());
    let clock = format!("{:.2} / {:.2} s", p.time, clip.duration());
    let strip = div()
        .absolute()
        .left_2()
        .bottom_2()
        .flex()
        .items_center()
        .gap_2();
    Some(
        strip
            .child(chip("pb-vp-play", play, !p.paused, t, cx, |v, _| {
                v.play.toggle()
            }))
            .child(chip("pb-vp-speed", speed, false, t, cx, |v, _| {
                v.play.speed = (v.play.speed + 1) % SPEEDS.len()
            }))
            .child(bar(p.progress(), t, cx))
            .child(div().text_xs().text_color(t.muted).child(clock)),
    )
}

/// The scrub bar; a press anywhere on it jumps there. Its bounds come
/// from the last paint, the only place GPUI hands them out.
fn bar(at: f32, t: &Tokens, cx: &mut Context<Viewport>) -> impl IntoElement {
    let area: Rc<Cell<Bounds<Pixels>>> = Rc::default();
    let seen_area = area.clone();
    let (rust, hover) = (t.rust, t.hover);
    div()
        .id("pb-vp-scrub")
        .debug_selector(|| "pb-vp-scrub".into())
        .relative()
        .w(px(180.0))
        .h(px(8.0))
        .rounded_sm()
        .bg(t.border)
        .hover(move |s| s.bg(hover))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |v, e: &gpui::MouseDownEvent, _, cx| {
                cx.stop_propagation();
                let b = area.get();
                let w = f32::from(b.size.width).max(1.0);
                v.scrub(f32::from(e.position.x - b.origin.x) / w, cx)
            }),
        )
        .child(seen(seen_area))
        .child(fill(at, rust))
}

/// Records the bar's bounds at each paint for the press handler.
fn seen(area: Rc<Cell<Bounds<Pixels>>>) -> impl IntoElement {
    canvas(move |b, _, _| area.set(b), |_, _, _, _| {})
        .absolute()
        .size_full()
}

fn fill(at: f32, color: gpui::Rgba) -> impl IntoElement {
    let el = div().absolute().left_0().top_0().h_full().rounded_sm();
    el.bg(color).w(gpui::relative(at))
}
