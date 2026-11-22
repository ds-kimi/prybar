use super::{Quat, Vec3};
use serde::Serialize;

/// Local (parent-space) transform of one bone at one frame.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize)]
pub struct Pose {
    pub position: Vec3,
    pub quat: Quat,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Animation {
    pub name: String,
    pub fps: f32,
    /// `STUDIO_*` animdesc flags (looping, delta, autoplay...).
    pub flags: i32,
    pub frame_count: usize,
    /// `frames[frame][bone]`. Empty when the data lives in an `.ani`
    /// file that was not supplied, so callers can still list the anim.
    pub frames: Vec<Vec<Pose>>,
    pub movements: Vec<Movement>,
    pub ik_rules: Vec<IkRule>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Movement {
    pub end_frame: i32,
    pub motion_flags: i32,
    pub v0: f32,
    pub v1: f32,
    pub angle: f32,
    pub vector: Vec3,
    pub position: Vec3,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct IkRule {
    pub index: i32,
    pub kind: i32,
    pub chain: i32,
    pub bone: i32,
    pub slot: i32,
    pub height: f32,
    pub radius: f32,
    pub floor: f32,
    pub position: Vec3,
    pub quat: Quat,
    /// start, peak, tail, end, as cycle fractions.
    pub influence: [f32; 4],
    pub contact: f32,
    pub drop: f32,
    pub top: f32,
    pub attachment: String,
}
