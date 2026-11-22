//! Drives sequence playback: picks a clip, moves the playhead each
//! frame and sends the posed mesh to the render thread.

use std::sync::Arc;
use std::time::Instant;

use gpui::{Context, Window};

use super::Viewport;
use crate::viewport::pose::{joints, skin, Clip};
use crate::viewport::render::Cmd;

impl Viewport {
    /// Plays sequence `seq` from the start; `None` goes back to the bind
    /// pose. A sequence without decoded frames shows the bind pose.
    pub fn play(&mut self, seq: Option<usize>, cx: &mut Context<Self>) {
        let ir = self.ir.as_ref();
        let clip = seq.and_then(|s| ir.and_then(|ir| Clip::new(ir, s)));
        self.play.clip = clip.map(Arc::new);
        (self.play.seq, self.play.time) = (seq, 0.0);
        (self.play.paused, self.play.tick) = (false, None);
        self.send();
        cx.notify();
    }

    /// Jumps to `at` (0 to 1) of the clip and holds there.
    pub fn scrub(&mut self, at: f32, cx: &mut Context<Self>) {
        let Some(clip) = &self.play.clip else {
            return;
        };
        self.play.time = at.clamp(0.0, 1.0) * clip.duration();
        (self.play.paused, self.play.tick) = (true, None);
        self.send();
        cx.notify();
    }

    /// Called at prepaint; keeps frames coming while a clip plays.
    pub(super) fn advance(&mut self, window: &mut Window) {
        let now = Instant::now();
        // A long gap (window hidden) must not skip half the clip.
        let dt = self
            .play
            .tick
            .map_or(0.0, |t| (now - t).as_secs_f32().min(0.25));
        if self.play.advance(dt) {
            self.play.tick = Some(now);
            self.send();
            window.request_animation_frame();
        } else {
            self.play.tick = None;
        }
    }

    /// Sends the mesh at the current pose; the next prepaint asks for a
    /// frame of it.
    pub(super) fn send(&mut self) {
        let (Some(bind), Some(worker)) = (&self.bind, &self.worker) else {
            return;
        };
        let mut scene = (**bind).clone();
        if let Some(clip) = &self.play.clip {
            let world = clip.at(self.play.time);
            skin(bind, &clip.skinning(&world), &mut scene.vertices);
            scene.bones = joints(&world, &clip.parents);
        }
        worker.send(Cmd::Scene(Box::new(scene)));
        self.last = None;
    }
}
