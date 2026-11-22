//! Camera moves that are not drags: wheel, trackpad, keys, and the
//! per-frame glide.

use std::time::Instant;

use glam::Vec2;
use gpui::{Context, Pixels, Point, Window};

use super::Viewport;

impl Viewport {
    /// Trackpad two-finger scroll: pans like a right drag would.
    pub(super) fn nudge(&mut self, d: Vec2) {
        let h = f32::from(self.area.size.height);
        if let Some(cam) = self.camera.as_mut() {
            cam.pan(d, h);
        }
    }

    /// Eased zoom by `factor` toward the point under `at`; repeated
    /// wheel ticks stack on the goal instead of restarting from here.
    pub(super) fn zoom_at(&mut self, at: Point<Pixels>, factor: f32) {
        let Some(cam) = self.camera else {
            return;
        };
        let r = cam.radius.max(1e-3);
        let goal = self.inertia.zoom_to.unwrap_or(cam.distance) * factor;
        self.inertia.zoom_to = Some(goal.clamp(r * 0.35, r * 12.0));
        self.inertia.toward = self.ndc(at).and_then(|(p, a)| cam.under(p, a));
        self.tick = None;
    }

    /// `F` frames the model from the current angle; `R` also resets
    /// the angle to the start view.
    pub(super) fn reframe(&mut self, reset: bool, cx: &mut Context<Self>) {
        let (Some(cam), Some(fit)) = (self.camera, self.fit) else {
            return;
        };
        self.inertia = Default::default();
        self.camera = Some(if reset { fit } else { cam.framed(&fit) });
        cx.notify();
    }

    /// Advances the glide by the real time since the last frame and asks
    /// for another frame while it lasts.
    pub(super) fn step(&mut self, window: &mut Window) {
        let now = Instant::now();
        let dt = self.tick.map_or(1.0 / 60.0, |t| (now - t).as_secs_f32());
        let h = f32::from(self.area.size.height);
        let moving = match self.camera.as_mut() {
            Some(cam) if self.drag.is_none() && self.inertia.moving() => {
                self.inertia.step(cam, dt, h)
            }
            _ => false,
        };
        // A stale tick after a pause would turn into one huge step.
        self.tick = moving.then_some(now);
        if moving {
            window.request_animation_frame();
        }
        self.advance(window);
    }
}
