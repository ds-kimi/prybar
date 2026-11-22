//! The GPUI side of the preview: loads the model off the UI thread,
//! asks the render thread for frames at the exact device size, and
//! paints the newest one.

mod aim;
mod drag;
mod frame;
mod input;
mod load;
mod overlay;
mod paint;
mod pick;
mod play;
#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::sync::Arc;

use gpui::{Bounds, Context, EventEmitter, FocusHandle, Pixels, RenderImage, Task};

use prybar_mdl::ir::ModelIr;

use super::camera::{Inertia, Orbit};
use super::mesh::{Pick, Scene};
use super::pose::Play;
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
    /// The fitted view, where `F` and `R` come back to.
    fit: Option<Orbit>,
    inertia: Inertia,
    drag: Option<drag::Drag>,
    /// Last glide frame; `None` restarts the clock after a pause.
    tick: Option<std::time::Instant>,
    clock_origin: Clock,
    /// Window position of the region, for cursor math.
    area: Bounds<Pixels>,
    focus: Option<FocusHandle>,
    /// Neutral grey behind the model instead of the theme surface, to
    /// judge silhouettes.
    pub backdrop: bool,
    /// The open model, kept to rebuild other body groups, skins and LODs
    /// and to build clips.
    pub ir: Option<Arc<ModelIr>>,
    /// The built mesh in bind pose; playback poses a copy of it.
    bind: Option<Arc<Scene>>,
    pub pick: Pick,
    pub play: Play,
    /// Only the newest variant build matters.
    rebuild: Option<Task<()>>,
}

/// Asks the host to switch between the side-by-side and full-size view.
pub struct Full;

impl EventEmitter<Full> for Viewport {}

/// Start of the drag sample clock; only differences matter.
struct Clock(std::time::Instant);

impl Default for Clock {
    fn default() -> Self {
        Self(std::time::Instant::now())
    }
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
        self.fit = None;
        self.inertia = Inertia::default();
        self.drag = None;
        self.last = None;
        self.drops.extend(self.image.take());
        (self.ir, self.bind, self.rebuild) = (None, None, None);
        (self.pick, self.play) = (Pick::default(), Play::default());
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
