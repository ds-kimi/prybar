//! The GPUI side of the preview: loads the model off the UI thread,
//! asks the render thread for frames at the exact device size, and
//! paints the newest one.

mod frame;
mod load;
mod paint;
#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::sync::Arc;

use gpui::{Context, RenderImage, Task};

use super::camera::Orbit;
use super::render::{Flags, Output, Request, Worker};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Status {
    #[default]
    Idle,
    Loading,
    Ready,
    Failed(String),
}

/// Theme colors the frame is drawn with, sRGB.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colors {
    pub clear: [f32; 4],
    pub grid: [f32; 4],
    pub wire: [f32; 4],
}

#[derive(Default)]
pub struct Viewport {
    path: Option<PathBuf>,
    worker: Option<Worker>,
    pub status: Status,
    image: Option<Arc<RenderImage>>,
    /// Replaced frames, freed from the sprite atlas at the next paint.
    drops: Vec<Arc<RenderImage>>,
    pub camera: Option<Orbit>,
    pub flags: Flags,
    last: Option<Request>,
    pub triangles: usize,
    pub ms: f32,
    tasks: Vec<Task<()>>,
}

impl Viewport {
    /// Switches to another model; the same path again does nothing, so
    /// calling this on every state change is cheap.
    pub fn show(&mut self, path: Option<PathBuf>, cx: &mut Context<Self>) {
        if path == self.path {
            return;
        }
        self.path = path.clone();
        self.camera = None;
        self.last = None;
        self.drops.extend(self.image.take());
        self.status = if path.is_some() {
            Status::Loading
        } else {
            Status::Idle
        };
        self.tasks.truncate(usize::from(self.worker.is_some()));
        if let Some(path) = path {
            self.tasks.push(load::start(path, cx));
        }
        cx.notify();
    }

    /// Called at prepaint with the device size of the region; only a
    /// change reaches the render thread.
    fn request(&mut self, width: u32, height: u32, colors: Colors) {
        let (Some(camera), Some(worker)) = (self.camera, &self.worker) else {
            return;
        };
        let req = Request {
            width,
            height,
            camera,
            flags: self.flags,
            clear: colors.clear,
            grid: colors.grid,
            wire: colors.wire,
        };
        if self.last != Some(req) {
            self.last = Some(req);
            worker.frame(req);
        }
    }

    fn accept(&mut self, out: Output, cx: &mut Context<Self>) {
        match out {
            Output::Ready { wire } => self.flags.wire &= wire,
            Output::Frame(f) => {
                self.ms = f.ms;
                self.status = Status::Ready;
                if let Some(img) = frame::image(f) {
                    self.drops.extend(self.image.replace(img));
                }
            }
            Output::Error(e) => self.status = Status::Failed(e),
        }
        cx.notify();
    }
}
