//! Offscreen wgpu renderer: scene in, BGRA pixels out.

#[cfg(test)]
mod bench;
mod frame;
mod gpu;
#[cfg(test)]
mod tests;
mod worker;

use std::time::Instant;

use super::camera::Orbit;
use super::mesh::Scene;

pub use worker::{Cmd, Output, Worker};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Flags {
    pub wire: bool,
    pub normals: bool,
    pub bones: bool,
}

/// Everything one frame depends on; equal requests give equal frames.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Request {
    /// Device pixels, already multiplied by the window scale factor.
    pub width: u32,
    pub height: u32,
    pub camera: Orbit,
    pub flags: Flags,
    /// sRGB colors from the theme.
    pub clear: [f32; 4],
    pub grid: [f32; 4],
    pub wire: [f32; 4],
}

pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub bgra: Vec<u8>,
    /// Render plus readback time, for the fps readout.
    pub ms: f32,
}

pub struct Renderer {
    gpu: gpu::Gpu,
    pipes: gpu::pipelines::Pipes,
    targets: Option<gpu::targets::Targets>,
    scene: Option<frame::upload::SceneBuffers>,
    materials: gpu::material::Materials,
}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        let gpu = gpu::open()?;
        let materials = gpu::material::Materials::new(&gpu);
        let pipes = gpu::pipelines::build(&gpu, &materials.layout);
        Ok(Self {
            gpu,
            pipes,
            targets: None,
            scene: None,
            materials,
        })
    }

    pub fn set_scene(&mut self, scene: &Scene) {
        self.scene = Some(frame::upload::upload(&self.gpu.device, scene));
    }

    /// Size is clamped to what the GPU allows, never zero.
    pub fn render(&mut self, req: &Request) -> Result<Frame, String> {
        let start = Instant::now();
        let req = self.fit(req);
        let (Some(t), Some(b)) = (&self.targets, &self.scene) else {
            return Err("No model loaded".into());
        };
        let q = &self.gpu.queue;
        let grid = frame::color::lines(&b.grid_points, frame::color::linear(req.grid));
        q.write_buffer(&self.gpu.globals, 0, &frame::color::globals(&req));
        q.write_buffer(&b.grid, 0, &grid);
        let mut enc = self.gpu.device.create_command_encoder(&Default::default());
        let binds = (&self.gpu.bind, &self.materials);
        frame::encode::pass(&mut enc, t, &self.pipes, b, &req, binds);
        let bgra = frame::readback::read(&self.gpu, t, enc)?;
        if let Some(e) = self.gpu.errors.lock().ok().and_then(|mut e| e.take()) {
            return Err(format!("GPU error: {e}"));
        }
        let ms = start.elapsed().as_secs_f32() * 1000.0;
        let (width, height) = (req.width, req.height);
        Ok(Frame {
            width,
            height,
            bgra,
            ms,
        })
    }

    /// Clamps the size and rebuilds the targets only when it changed.
    fn fit(&mut self, req: &Request) -> Request {
        let max = self.gpu.device.limits().max_texture_dimension_2d;
        let (w, h) = (req.width.clamp(1, max), req.height.clamp(1, max));
        let stale = self
            .targets
            .as_ref()
            .is_none_or(|t| (t.width, t.height) != (w, h));
        if stale {
            self.targets = Some(gpu::targets::Targets::new(&self.gpu.device, w, h));
        }
        Request {
            width: w,
            height: h,
            ..*req
        }
    }
}
