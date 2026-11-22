//! Drags that keep tracking once the cursor leaves the viewport, and the
//! glide that follows them.

use glam::Vec2;
use gpui::{DispatchPhase, Entity, MouseButton, MouseMoveEvent, MouseUpEvent, Window};

use super::Viewport;
use crate::viewport::camera::Inertia;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Orbit,
    Pan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Drag {
    pub mode: Mode,
    pub last: Vec2,
    /// Seconds and cumulative travel, for the release velocity.
    pub samples: Vec<(f32, Vec2)>,
}

/// Window-wide handlers for this frame, so a drag that leaves the
/// region neither sticks nor stops tracking.
pub fn listen(window: &mut Window, this: Entity<Viewport>) {
    let moves = this.clone();
    window.on_mouse_event(move |e: &MouseMoveEvent, phase, _, cx| {
        if phase == DispatchPhase::Bubble && e.pressed_button.is_some() {
            let p = Vec2::new(f32::from(e.position.x), f32::from(e.position.y));
            moves.update(cx, |v, cx| v.drag_to(p, cx));
        }
    });
    window.on_mouse_event(move |e: &MouseUpEvent, phase, _, cx| {
        let ours = matches!(
            e.button,
            MouseButton::Left | MouseButton::Right | MouseButton::Middle
        );
        if phase == DispatchPhase::Bubble && ours {
            this.update(cx, |v, cx| v.let_go(cx));
        }
    });
}

impl Viewport {
    fn drag_to(&mut self, p: Vec2, cx: &mut gpui::Context<Self>) {
        let (Some(d), Some(cam)) = (self.drag.as_mut(), self.camera.as_mut()) else {
            return;
        };
        let delta = p - d.last;
        d.last = p;
        let now = self.clock_origin.0.elapsed().as_secs_f32();
        let total = d.samples.last().map_or(Vec2::ZERO, |s| s.1) + delta;
        d.samples.push((now, total));
        if d.samples.len() > 32 {
            d.samples.remove(0);
        }
        match d.mode {
            Mode::Orbit => cam.rotate(delta),
            Mode::Pan => cam.pan(delta, f32::from(self.area.size.height)),
        }
        cx.notify();
    }

    fn let_go(&mut self, cx: &mut gpui::Context<Self>) {
        let Some(d) = self.drag.take() else {
            return;
        };
        let v = Inertia::release(&d.samples, self.clock_origin.0.elapsed().as_secs_f32());
        match d.mode {
            Mode::Orbit => self.inertia.spin = v,
            Mode::Pan => self.inertia.slide = v,
        }
        self.tick = None;
        cx.notify();
    }
}
