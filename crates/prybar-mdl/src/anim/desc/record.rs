use super::{ikrule, movement};
use crate::anim::decode::{frames, rest_frames, AnimData, Layout};
use crate::ir::{Animation, Bone, Pose};
use crate::read::{name_at, Source, Words};
use crate::{Error, Result};

/// `mstudioanimdesc_t` size, same from v44 to v49.
pub const STRIDE: usize = 100;
const DELTA: i32 = 0x4;
const ALL_ZEROS: i32 = 0x20;
/// Per-frame anims are a later (v52+) storage; no v44-49 model uses it.
const FRAME_ANIM: i32 = 0x40;

pub fn animation(r: &Source, d: &AnimData, at: usize, bones: &[Bone]) -> Result<Animation> {
    let w = Words::read(r, at, STRIDE / 4)?;
    let frame_count = usize::try_from(w.i(4)).map_err(|_| Error::Invalid {
        what: "anim frame count",
        value: w.i(4).into(),
    })?;
    let layout = Layout {
        at,
        frame_count,
        block: w.i(13),
        index: w.i(14),
        section_index: w.i(20),
        section_frames: w.i(21),
    };
    Ok(Animation {
        name: name_at(r, at, w.i(1))?,
        fps: w.f(2),
        flags: w.i(3),
        frame_count,
        frames: poses(d, &layout, bones, w.i(3))?,
        movements: movement::movements(r, at, w.i(5), w.i(6))?,
        ik_rules: ikrule::ik_rules(r, d, &w, at)?,
    })
}

fn poses(d: &AnimData, l: &Layout, bones: &[Bone], flags: i32) -> Result<Vec<Vec<Pose>>> {
    if flags & ALL_ZEROS != 0 {
        return rest_frames(d, l.frame_count, bones);
    }
    if flags & FRAME_ANIM != 0 {
        return Ok(Vec::new());
    }
    frames(d, l, bones, flags & DELTA != 0)
}
