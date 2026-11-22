use std::sync::Arc;
use std::time::Instant;

use super::Clip;

/// Speeds the speed chip cycles through, starting at real time.
pub const SPEEDS: [f32; 4] = [1.0, 2.0, 0.25, 0.5];

/// What the viewport is playing and where it is in it.
#[derive(Debug, Default)]
pub struct Play {
    pub clip: Option<Arc<Clip>>,
    /// The sequence picked, even when it has no frames to play.
    pub seq: Option<usize>,
    pub time: f32,
    /// Index into [`SPEEDS`].
    pub speed: usize,
    pub paused: bool,
    /// Last playback frame; `None` restarts the clock after a pause.
    pub tick: Option<Instant>,
}

impl Play {
    pub fn rate(&self) -> f32 {
        SPEEDS[self.speed % SPEEDS.len()]
    }

    /// Moves time on by `dt` seconds; `false` when nothing plays. A
    /// sequence that does not loop stops on its last frame.
    pub fn advance(&mut self, dt: f32) -> bool {
        let Some(clip) = &self.clip else {
            return false;
        };
        if self.paused {
            return false;
        }
        let end = clip.duration();
        self.time += dt.max(0.0) * self.rate();
        if self.time >= end {
            if clip.looping {
                self.time = self.time.rem_euclid(end);
            } else {
                (self.time, self.paused) = (end, true);
            }
        }
        true
    }

    /// Play or pause; playing a finished clip starts it over.
    pub fn toggle(&mut self) {
        self.paused = !self.paused;
        self.tick = None;
        let done = self
            .clip
            .as_ref()
            .is_some_and(|c| self.time >= c.duration());
        if !self.paused && done {
            self.time = 0.0;
        }
    }

    /// Where the playhead is, 0 to 1.
    pub fn progress(&self) -> f32 {
        self.clip
            .as_ref()
            .map_or(0.0, |c| (self.time / c.duration()).clamp(0.0, 1.0))
    }
}
