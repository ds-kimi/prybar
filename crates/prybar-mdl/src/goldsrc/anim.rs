use crate::anim::math::euler_to_quat;
use crate::ir::{Bone, Pose};
use crate::read::Source;
use crate::{Error, Result};

/// `mstudioanim_t` per bone: six u16 offsets (x y z, then roll pitch
/// yaw), each relative to the struct; 0 means the channel never moves.
pub fn frames(r: &Source, at: usize, bones: &[Bone], count: usize) -> Result<Vec<Vec<Pose>>> {
    r.charge(count.saturating_mul(bones.len().max(1)).saturating_mul(24))?;
    let mut out = vec![Vec::with_capacity(bones.len()); count];
    for (i, bone) in bones.iter().enumerate() {
        for (frame, c) in out
            .iter_mut()
            .zip(bone_channels(r, at + i * 12, bone, count)?)
        {
            frame.push(pose(c));
        }
    }
    Ok(out)
}

/// Values are deltas scaled onto the bone's rest pose.
fn bone_channels(r: &Source, base: usize, bone: &Bone, count: usize) -> Result<Vec<[f32; 6]>> {
    let offs: Vec<u16> = r.array(base, 6, 6)?;
    let rest = [bone.position, bone.rotation].concat();
    let scale = [bone.position_scale, bone.rotation_scale].concat();
    let mut chan = vec![[0.0f32; 6]; count];
    for (j, &off) in offs.iter().enumerate() {
        let raw = match off {
            0 => vec![0; count],
            o => channel(r, base + usize::from(o), count)?,
        };
        for (c, v) in chan.iter_mut().zip(raw) {
            c[j] = rest[j] + f32::from(v) * scale[j];
        }
    }
    Ok(chan)
}

/// Run-length stream: a header word holds `valid` and `total` bytes, then
/// `valid` values; frames past `valid` repeat the last one. With
/// `valid == 0` that "last one" is the header word itself, exactly as
/// the engine reads it, so the output matches what the game plays.
fn channel(r: &Source, mut at: usize, count: usize) -> Result<Vec<i16>> {
    let mut out = Vec::with_capacity(count);
    while out.len() < count {
        let (valid, total) = (
            usize::from(r.read::<u8>(at)?),
            usize::from(r.read::<u8>(at + 1)?),
        );
        if total == 0 {
            return Err(Error::Invalid {
                what: "empty animation run at",
                value: at as i64,
            });
        }
        let values: Vec<i16> = r.array(at, valid + 1, valid + 1)?;
        for k in 0..total.min(count - out.len()) {
            out.push(values[if k < valid { k + 1 } else { valid }]);
        }
        at += 2 * (valid + 1);
    }
    Ok(out)
}

fn pose(c: [f32; 6]) -> Pose {
    Pose {
        position: [c[0], c[1], c[2]],
        quat: euler_to_quat([c[3], c[4], c[5]]),
    }
}
