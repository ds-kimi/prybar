//! Mouse, trackpad and keys on the viewport. Left drag orbits; right,
//! middle or shift drag pans; wheel and pinch zoom toward the cursor;
//! two-finger trackpad scroll pans.

use glam::Vec2;
use gpui::{
    prelude::*, Context, Div, MouseButton, MouseDownEvent, PinchEvent, Pixels, Point, ScrollDelta,
    ScrollEvent,
};

use super::drag::{Drag, Mode};
use super::{Full, Viewport};
use crate::actions::{FrameModel, ResetView, ToggleBones, ToggleFullView, TogglePlay, ToggleWire};

pub fn attach(el: Div, cx: &mut Context<Viewport>) -> Div {
    let down =
        |b| cx.listener(move |v: &mut Viewport, e: &MouseDownEvent, w, cx| v.press(b, e, w, cx));
    el.on_mouse_down(MouseButton::Left, down(MouseButton::Left))
        .on_mouse_down(MouseButton::Right, down(MouseButton::Right))
        .on_mouse_down(MouseButton::Middle, down(MouseButton::Middle))
        .on_scroll(cx.listener(|v, e: &ScrollEvent, _, cx| {
            match e.delta {
                // Trackpads send pixels: two fingers pan, cmd or ctrl
                // turns them into zoom like most 3D tools.
                ScrollDelta::Pixels(d) if !(e.modifiers.platform || e.modifiers.control) => {
                    v.nudge(Vec2::new(f32::from(d.x), f32::from(d.y)))
                }
                d => {
                    let notches = f32::from(d.pixel_delta(gpui::px(16.0)).y) / 16.0;
                    v.zoom_at(e.position, 0.9f32.powf(notches))
                }
            }
            cx.notify()
        }))
        .on_pinch(cx.listener(|v, e: &PinchEvent, _, cx| {
            v.zoom_at(e.position, 1.0 / (1.0 + e.delta).max(0.1));
            cx.notify()
        }))
}

pub fn keys(el: Div, cx: &mut Context<Viewport>) -> Div {
    el.key_context("Viewport")
        .on_action(cx.listener(|v, _: &FrameModel, _, cx| v.reframe(false, cx)))
        .on_action(cx.listener(|v, _: &ResetView, _, cx| v.reframe(true, cx)))
        .on_action(cx.listener(|_, _: &ToggleFullView, _, cx| cx.emit(Full)))
        .on_action(cx.listener(|v, _: &ToggleWire, _, cx| {
            v.flags.wire = !v.flags.wire;
            cx.notify()
        }))
        .on_action(cx.listener(|v, _: &ToggleBones, _, cx| {
            v.flags.bones = !v.flags.bones;
            cx.notify()
        }))
        .on_action(cx.listener(|v, _: &TogglePlay, _, cx| {
            v.play.toggle();
            cx.notify()
        }))
}

impl Viewport {
    /// Starts a drag; a double-click instead moves the orbit center to
    /// the point under the cursor.
    fn press(
        &mut self,
        b: MouseButton,
        e: &MouseDownEvent,
        w: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(f) = &self.focus {
            f.focus(w, cx);
        }
        self.inertia = Default::default();
        let p = Vec2::new(f32::from(e.position.x), f32::from(e.position.y));
        if b == MouseButton::Left && e.click_count == 2 {
            let at = self.ndc(e.position);
            if let (Some(cam), Some(at)) = (self.camera.as_mut(), at) {
                if let Some(p) = cam.under(at.0, at.1) {
                    cam.target = p;
                }
            }
            return cx.notify();
        }
        let pan = b != MouseButton::Left || e.modifiers.shift;
        let mode = if pan { Mode::Pan } else { Mode::Orbit };
        self.drag = Some(Drag {
            mode,
            last: p,
            samples: Vec::new(),
        });
    }

    /// Normalized device point and aspect for a window position.
    pub(super) fn ndc(&self, p: Point<Pixels>) -> Option<(Vec2, f32)> {
        let a = self.area;
        let (w, h) = (f32::from(a.size.width), f32::from(a.size.height));
        if w < 1.0 || h < 1.0 {
            return None;
        }
        let x = (f32::from(p.x - a.origin.x) / w) * 2.0 - 1.0;
        let y = 1.0 - (f32::from(p.y - a.origin.y) / h) * 2.0;
        Some((Vec2::new(x, y), w / h))
    }
}
