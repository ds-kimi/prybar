use super::frames::rest;
use super::value::{axis_values, Step};
use crate::anim::math::euler_to_quat;
use crate::anim::packed::{quat48, quat64, vector48};
use crate::ir::{Bone, Pose, Quat, Vec3};
use crate::Result;
use prybar_io::Reader;

const RAW_POS: u8 = 0x01;
const RAW_ROT: u8 = 0x02;
const ANIM_POS: u8 = 0x04;
const ANIM_ROT: u8 = 0x08;
const DELTA: u8 = 0x10;
const RAW_ROT2: u8 = 0x20;
pub const IDENTITY: Quat = [0.0, 0.0, 0.0, 1.0];

/// Decodes one `mstudioanim_t` record at one frame. Rotation data comes
/// first and the position data follows all rotation blocks present.
pub fn bone_pose(r: &Reader, at: usize, bone: &Bone, frame: &mut Step) -> Result<Pose> {
    let flags: u8 = r.read(at + 1)?;
    let data = at + 4;
    let rot_len = [(RAW_ROT, 6), (RAW_ROT2, 8), (ANIM_ROT, 6)]
        .iter()
        .filter(|(f, _)| flags & f != 0)
        .map(|(_, n)| n)
        .sum::<usize>();
    Ok(Pose {
        quat: rotation(r, data, flags, bone, frame)?,
        position: position(r, data + rot_len, flags, bone, frame)?,
    })
}

fn rotation(r: &Reader, data: usize, flags: u8, bone: &Bone, frame: &mut Step) -> Result<Quat> {
    let delta = flags & DELTA != 0;
    if flags & RAW_ROT != 0 {
        return quat48(r, data);
    }
    if flags & RAW_ROT2 != 0 {
        return quat64(r, data);
    }
    if flags & ANIM_ROT == 0 {
        return Ok(if delta { IDENTITY } else { rest(bone).quat });
    }
    let raw = axis_values(r, data, frame)?;
    let base = if delta { [0.0; 3] } else { bone.rotation };
    Ok(euler_to_quat(scaled(raw, bone.rotation_scale, base)))
}

fn position(r: &Reader, data: usize, flags: u8, bone: &Bone, frame: &mut Step) -> Result<Vec3> {
    let delta = flags & DELTA != 0;
    if flags & RAW_POS != 0 {
        return vector48(r, data);
    }
    if flags & ANIM_POS == 0 {
        return Ok(if delta { [0.0; 3] } else { rest(bone).position });
    }
    let raw = axis_values(r, data, frame)?;
    let base = if delta { [0.0; 3] } else { bone.position };
    Ok(scaled(raw, bone.position_scale, base))
}

fn scaled(raw: [f32; 3], scale: Vec3, base: Vec3) -> Vec3 {
    [0, 1, 2].map(|i| base[i] + raw[i] * scale[i])
}
