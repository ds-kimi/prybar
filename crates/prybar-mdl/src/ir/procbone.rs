use super::{Quat, Vec3};
use serde::Serialize;

/// `mstudiobone_t::proctype` data.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ProcBone {
    AxisInterp,
    QuatInterp {
        control: i32,
        triggers: Vec<QuatTrigger>,
    },
    AimAtBone(AimAt),
    AimAtAttachment(AimAt),
    Jiggle(Jiggle),
    Unknown(i32),
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct QuatTrigger {
    /// Radians; the file stores its inverse.
    pub tolerance: f32,
    pub trigger: Quat,
    pub position: Vec3,
    pub quat: Quat,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct AimAt {
    pub parent: i32,
    /// Bone or attachment index, depending on the variant.
    pub aim: i32,
    pub aim_vector: Vec3,
    pub up_vector: Vec3,
    pub base_pos: Vec3,
}

/// `mstudiojigglebone_t`: flags then its 34 floats in file order
/// (length, tip mass, yaw/pitch/along stiffness and damping, limits,
/// base spring, boing), kept raw for the QC writer.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Jiggle {
    pub flags: i32,
    pub params: Vec<f32>,
}
