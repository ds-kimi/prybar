use glam::Mat4;
use prybar_mdl::ir::ModelIr;

use super::world::{inverse_bind, parent, world};

/// `STUDIO_LOOPING` on a sequence.
const LOOPING: i32 = 0x0001;
/// A hostile file can claim any frame count; past this a preview clip
/// would only eat memory.
const MAX_FRAMES: usize = 4096;

/// One sequence ready to sample: bone matrices for every frame.
#[derive(Debug, Clone)]
pub struct Clip {
    pub fps: f32,
    pub looping: bool,
    frames: Vec<Vec<Mat4>>,
    inverse: Vec<Mat4>,
    pub parents: Vec<Option<usize>>,
}

impl Clip {
    /// `None` when the sequence has no decoded frames (data in a missing
    /// `.ani`, or no bones), so the caller can say so instead of playing.
    pub fn new(ir: &ModelIr, seq: usize) -> Option<Self> {
        let s = ir.sequences.get(seq)?;
        let anim = usize::try_from(*s.anims.first()?).ok()?;
        let a = ir.animations.get(anim)?;
        if a.frames.is_empty() || ir.bones.is_empty() {
            return None;
        }
        let frames = a.frames.iter().take(MAX_FRAMES);
        let fps = if a.fps.is_finite() && a.fps > 0.0 {
            a.fps.min(240.0)
        } else {
            30.0
        };
        Some(Self {
            fps,
            looping: s.flags & LOOPING != 0,
            frames: frames.map(|f| world(&ir.bones, f)).collect(),
            inverse: ir
                .bones
                .iter()
                .map(|b| inverse_bind(&b.pose_to_bone))
                .collect(),
            parents: ir
                .bones
                .iter()
                .enumerate()
                .map(|(i, b)| parent(b, i))
                .collect(),
        })
    }

    /// Seconds from the first to the last frame; never zero.
    pub fn duration(&self) -> f32 {
        self.frames.len().saturating_sub(1).max(1) as f32 / self.fps
    }

    /// Bone-to-model matrices at `t` seconds, blended between the two
    /// nearest frames so slow motion stays smooth.
    pub fn at(&self, t: f32) -> Vec<Mat4> {
        let last = self.frames.len().saturating_sub(1);
        let f = (t.max(0.0) * self.fps).min(last as f32);
        let (i, k) = (f.floor() as usize, f.fract());
        let (a, b) = (&self.frames[i.min(last)], &self.frames[(i + 1).min(last)]);
        a.iter()
            .zip(b)
            .map(|(a, b)| *a * (1.0 - k) + *b * k)
            .collect()
    }

    /// Model-to-posed matrices: what each vertex weight applies.
    pub fn skinning(&self, world: &[Mat4]) -> Vec<Mat4> {
        world
            .iter()
            .zip(&self.inverse)
            .map(|(w, i)| *w * *i)
            .collect()
    }
}
