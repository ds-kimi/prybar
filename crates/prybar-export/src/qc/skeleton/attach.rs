use crate::qc::fmt::{num, quote, vec3};
use prybar_mdl::ir::Mat3x4;
use prybar_mdl::ModelIr;
use std::fmt::Write;

const ATTACHMENT_FLAG_WORLD_ALIGN: i32 = 0x10000;

pub fn write(s: &mut String, ir: &ModelIr) {
    for a in &ir.attachments {
        let bone = usize::try_from(a.bone).ok().and_then(|b| ir.bones.get(b));
        let [p, y, r] = angles(&a.local);
        let _ = write!(
            s,
            "$attachment {} {} {} rotate {} {} {}",
            quote(&a.name),
            quote(bone.map_or("", |b| b.name.as_str())),
            vec3([a.local[0][3], a.local[1][3], a.local[2][3]]),
            num(p),
            num(y),
            num(r),
        );
        if a.flags & ATTACHMENT_FLAG_WORLD_ALIGN != 0 {
            s.push_str(" world_align");
        }
        s.push('\n');
    }
}

/// Source's `MatrixAngles`: pitch yaw roll in degrees, the order the
/// `rotate` option reads; gimbal lock folds roll into yaw.
pub fn angles(m: &Mat3x4) -> [f32; 3] {
    let (f0, f1, f2) = (m[0][0], m[1][0], m[2][0]);
    let xy = (f0 * f0 + f1 * f1).sqrt();
    let pitch = (-f2).atan2(xy);
    let (yaw, roll) = if xy > 0.001 {
        (f1.atan2(f0), m[2][1].atan2(m[2][2]))
    } else {
        ((-m[0][1]).atan2(m[1][1]), 0.0)
    };
    [pitch, yaw, roll].map(f32::to_degrees)
}
