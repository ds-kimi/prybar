//! Procedural bone rules (`mstudiobone_t::proctype`). Quaternion and aim
//! rules become a VRD file, jiggle bones a QC `$jigglebone`.

mod aim;
mod jiggle;
mod quat;

use crate::ir::ProcBone;
use crate::read::{offset, Source};
use crate::Result;

const AXIS_INTERP: i32 = 1;
const QUAT_INTERP: i32 = 2;
const AIM_AT_BONE: i32 = 3;
const AIM_AT_ATTACH: i32 = 4;
const JIGGLE: i32 = 5;

/// `index` is relative to the bone; 0 means the bone has no rule even
/// when a stale type is left in the struct.
pub fn parse(
    r: &Source,
    bone_at: usize,
    (kind, index): (i32, i32),
    version: i32,
) -> Result<Option<ProcBone>> {
    if kind == 0 || index == 0 {
        return Ok(None);
    }
    let at = offset(bone_at, index)?;
    Ok(Some(match kind {
        AXIS_INTERP => ProcBone::AxisInterp,
        QUAT_INTERP => quat::quat_interp(r, at)?,
        AIM_AT_BONE => ProcBone::AimAtBone(aim::aim_at(r, at)?),
        AIM_AT_ATTACH => ProcBone::AimAtAttachment(aim::aim_at(r, at)?),
        JIGGLE => ProcBone::Jiggle(jiggle::jiggle(r, at, version)?),
        other => ProcBone::Unknown(other),
    }))
}
