use super::bone::{bone_pose, IDENTITY};
use super::chain::records;
use super::data::AnimData;
use super::section::{locate, Layout};
use super::value::Step;
use crate::anim::root::root_rest;
use crate::ir::{Bone, Pose};
use crate::Result;

/// `(block, index)` of the stream a frame reads; sections change it.
type DataKey = (i32, i32);

/// Decodes every frame, or returns an empty list when some frame lives in
/// an `.ani` file that was not supplied. `delta` anims default missing
/// bones to the identity instead of the bind pose.
pub fn frames(d: &AnimData, l: &Layout, bones: &[Bone], delta: bool) -> Result<Vec<Vec<Pose>>> {
    d.charge(l.frame_count.saturating_mul(bones.len().max(1)))?;
    let mut out = Vec::with_capacity(l.frame_count);
    let mut cached: Option<(DataKey, Vec<Option<usize>>)> = None;
    let mut step = Step::default();
    for frame in 0..l.frame_count {
        let (block, index, local) = locate(&d.mdl.reader, l, frame)?;
        let Some((r, start)) = d.locate(block, index, l.at)? else {
            return Ok(Vec::new());
        };
        if cached.as_ref().map(|c| c.0) != Some((block, index)) {
            cached = Some(((block, index), records(&r.reader, start, bones.len())?));
            step.reset();
        }
        step.frame = local;
        let chain = cached.as_ref().map(|c| &c.1[..]).unwrap_or_default();
        out.push(frame_poses(&r.reader, chain, bones, &mut step, delta)?);
    }
    Ok(out)
}

fn frame_poses(
    r: &prybar_io::Reader,
    chain: &[Option<usize>],
    bones: &[Bone],
    step: &mut Step,
    delta: bool,
) -> Result<Vec<Pose>> {
    bones
        .iter()
        .zip(chain)
        .map(|(bone, rec)| match rec {
            Some(at) => bone_pose(r, *at, bone, step),
            None if delta => Ok(Pose {
                position: [0.0; 3],
                quat: IDENTITY,
            }),
            None => Ok(rest(bone)),
        })
        .collect()
}

/// Bind pose, root bones in animation space like decoded data.
pub fn rest(bone: &Bone) -> Pose {
    if bone.parent < 0 {
        return root_rest(bone.position, bone.quat);
    }
    Pose {
        position: bone.position,
        quat: bone.quat,
    }
}

/// Bind pose for every frame, used by anims flagged as all-zero.
pub fn rest_frames(d: &AnimData, frame_count: usize, bones: &[Bone]) -> Result<Vec<Vec<Pose>>> {
    d.charge(frame_count.saturating_mul(bones.len().max(1)))?;
    let pose: Vec<Pose> = bones.iter().map(rest).collect();
    Ok(vec![pose; frame_count])
}
