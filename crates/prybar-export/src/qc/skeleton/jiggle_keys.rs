//! `$jigglebone` options and where their values sit in the 34 floats of
//! `mstudiojigglebone_t`. Angles are stored in radians; the boing impact
//! angle is stored as its cosine.

pub struct Key {
    pub name: &'static str,
    pub params: &'static [usize],
    /// Constraint bit the option depends on, 0 when always written.
    pub flag: i32,
    pub convert: fn(f32) -> f32,
}

const YAW: i32 = 0x04;
const PITCH: i32 = 0x08;
const ANGLE: i32 = 0x10;

const fn key(name: &'static str, params: &'static [usize], flag: i32) -> Key {
    Key {
        name,
        params,
        flag,
        convert: same,
    }
}

const fn angle(name: &'static str, params: &'static [usize], flag: i32) -> Key {
    Key {
        name,
        params,
        flag,
        convert: f32::to_degrees,
    }
}

fn same(v: f32) -> f32 {
    v
}

fn cos_to_degrees(v: f32) -> f32 {
    v.clamp(-1.0, 1.0).acos().to_degrees()
}

pub const FLEXIBLE: &[Key] = &[
    key("length", &[0], 0),
    key("tip_mass", &[1], 0),
    key("yaw_stiffness", &[2], 0),
    key("yaw_damping", &[3], 0),
    key("pitch_stiffness", &[4], 0),
    key("pitch_damping", &[5], 0),
    key("along_stiffness", &[6], 0),
    key("along_damping", &[7], 0),
    angle("angle_constraint", &[8], ANGLE),
    angle("yaw_constraint", &[9, 10], YAW),
    key("yaw_friction", &[11], YAW),
    key("yaw_bounce", &[12], YAW),
    angle("pitch_constraint", &[13, 14], PITCH),
    key("pitch_friction", &[15], PITCH),
    key("pitch_bounce", &[16], PITCH),
];

pub const RIGID: &[Key] = &[
    key("length", &[0], 0),
    key("tip_mass", &[1], 0),
    angle("angle_constraint", &[8], ANGLE),
    angle("yaw_constraint", &[9, 10], YAW),
    key("yaw_friction", &[11], YAW),
    angle("pitch_constraint", &[13, 14], PITCH),
    key("pitch_friction", &[15], PITCH),
];

pub const BASE: &[Key] = &[
    key("base_mass", &[17], 0),
    key("stiffness", &[18], 0),
    key("damping", &[19], 0),
    key("left_constraint", &[20, 21], 0),
    key("left_friction", &[22], 0),
    key("up_constraint", &[23, 24], 0),
    key("up_friction", &[25], 0),
    key("forward_constraint", &[26, 27], 0),
    key("forward_friction", &[28], 0),
];

pub const BOING: &[Key] = &[
    key("impact_speed", &[29], 0),
    Key {
        name: "impact_angle",
        params: &[30],
        flag: 0,
        convert: cos_to_degrees,
    },
    key("damping_rate", &[31], 0),
    key("frequency", &[32], 0),
    key("amplitude", &[33], 0),
];
