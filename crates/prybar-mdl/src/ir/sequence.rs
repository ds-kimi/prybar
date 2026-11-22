use super::Vec3;
use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Sequence {
    pub name: String,
    pub activity_name: String,
    pub flags: i32,
    pub activity: i32,
    pub activity_weight: i32,
    pub events: Vec<Event>,
    pub bb_min: Vec3,
    pub bb_max: Vec3,
    /// Blend grid size; `anims` holds `group_size[0] * group_size[1]`
    /// animation indices, row-major.
    pub group_size: [i32; 2],
    pub anims: Vec<i16>,
    pub param_index: [i32; 2],
    pub param_start: [f32; 2],
    pub param_end: [f32; 2],
    pub param_parent: i32,
    pub fade_in: f32,
    pub fade_out: f32,
    pub entry_node: i32,
    pub exit_node: i32,
    pub node_flags: i32,
    pub entry_phase: f32,
    pub exit_phase: f32,
    pub last_frame: f32,
    pub next_seq: i32,
    pub pose: i32,
    pub auto_layers: Vec<AutoLayer>,
    /// One weight per bone; empty when the model has no bones.
    pub bone_weights: Vec<f32>,
    pub pose_keys: Vec<f32>,
    pub ik_locks: Vec<IkLock>,
    pub key_values: String,
    pub cycle_pose: i32,
    pub activity_modifiers: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Event {
    pub cycle: f32,
    pub event: i32,
    pub kind: i32,
    pub options: String,
    pub name: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct AutoLayer {
    pub sequence: i16,
    pub pose: i16,
    pub flags: i32,
    /// start, peak, tail, end.
    pub ramp: [f32; 4],
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct IkLock {
    pub chain: i32,
    pub pos_weight: f32,
    pub local_q_weight: f32,
    pub flags: i32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct PoseParam {
    pub name: String,
    pub flags: i32,
    pub start: f32,
    pub end: f32,
    pub loop_range: f32,
}
