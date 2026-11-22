use super::{Mat3x4, ProcBone, Quat, Vec3};
use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Bone {
    pub name: String,
    /// -1 for a root bone.
    pub parent: i32,
    pub position: Vec3,
    pub quat: Quat,
    pub rotation: Vec3,
    pub position_scale: Vec3,
    pub rotation_scale: Vec3,
    pub pose_to_bone: Mat3x4,
    pub flags: i32,
    pub surface_prop: String,
    pub contents: i32,
    pub physics_bone: i32,
    pub procedural: Option<ProcBone>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Attachment {
    pub name: String,
    pub bone: i32,
    pub flags: i32,
    pub local: Mat3x4,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct HitboxSet {
    pub name: String,
    pub hitboxes: Vec<Hitbox>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Hitbox {
    pub name: String,
    pub bone: i32,
    pub group: i32,
    pub min: Vec3,
    pub max: Vec3,
}

/// `$ikchain`: studiomdl rebuilds the links from the end bone, so only
/// that bone and the knee direction of the first link are kept.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct IkChain {
    pub name: String,
    pub bone: i32,
    pub knee: Vec3,
}
