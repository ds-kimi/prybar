//! Wheel and pinch zoom toward the cursor, two-finger scroll and left
//! drag pan with a glide on release, double-click fits.

use glam::Vec2;
use gpui::{
    prelude::*, Context, Div, MouseButton, MouseDownEvent, PinchEvent, ScrollDelta, ScrollEvent,
};

use super::canvas::Drag;
use super::{Pan, TextureView};
use crate::viewport::camera::Inertia;

pub fn attach(el: Div, cx: &mut Context<TextureView>) -> Div {
    el.on_mouse_down(
        MouseButton::Left,
        cx.listener(|v, e: &MouseDownEvent, w, cx| v.press(e, w, cx)),
    )
    .on_scroll(cx.listener(|v, e: &ScrollEvent, _, cx| {
        match e.delta {
            ScrollDelta::Pixels(d) if !(e.modifiers.platform || e.modifiers.control) => {
                v.pan.offset += Vec2::new(f32::from(d.x), f32::from(d.y));
            }
            d => {
                let notches = f32::from(d.pixel_delta(gpui::px(16.0)).y) / 16.0;
                v.zoom(e.position, 1.1f32.powf(notches));
            }
        }
        cx.notify()
    }))
    .on_pinch(cx.listener(|v, e: &PinchEvent, _, cx| {
        v.zoom(e.position, (1.0 + e.delta).max(0.1));
        cx.notify()
    }))
}

impl TextureView {
    fn press(&mut self, e: &MouseDownEvent, w: &mut gpui::Window, cx: &mut Context<Self>) {
        if let Some(f) = &self.focus {
            f.focus(w, cx);
        }
        self.pan.slide = Vec2::ZERO;
        if e.click_count == 2 {
            self.pan = Pan::default();
            return cx.notify();
        }
        let last = Vec2::new(f32::from(e.position.x), f32::from(e.position.y));
        self.drag = Some(Drag {
            last,
            samples: Vec::new(),
            start: std::time::Instant::now(),
        });
    }

    pub(super) fn drag_to(&mut self, p: Vec2, cx: &mut Context<Self>) {
        let Some(d) = self.drag.as_mut() else { return };
        let delta = p - d.last;
        d.last = p;
        let total = d.samples.last().map_or(Vec2::ZERO, |s| s.1) + delta;
        d.samples.push((d.start.elapsed().as_secs_f32(), total));
        if d.samples.len() > 32 {
            d.samples.remove(0);
        }
        self.pan.offset += delta;
        cx.notify();
    }

    pub(super) fn let_go(&mut self, cx: &mut Context<Self>) {
        let Some(d) = self.drag.take() else { return };
        self.pan.slide = Inertia::release(&d.samples, d.start.elapsed().as_secs_f32());
        self.tick = None;
        cx.notify();
    }
}
