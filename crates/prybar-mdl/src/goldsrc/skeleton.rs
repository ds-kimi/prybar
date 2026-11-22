use super::header::{fixed_name, GoldHeader, ATTACHMENTS, BONES, HITBOXES};
use crate::anim::math::euler_to_quat;
use crate::ir::{Attachment, Bone, Hitbox, HitboxSet};
use crate::read::{table, Source, Words};
use crate::Result;

/// `mstudiobone_t`: a 32-byte name then 20 words.
pub fn bones(r: &Source, h: &GoldHeader) -> Result<Vec<Bone>> {
    let (count, index) = h.span(BONES);
    table(r, 0, count, index, 112)?
        .into_iter()
        .map(|at| {
            let w = Words::read(r, at + 32, 20)?;
            let rotation = w.v3(11);
            Ok(Bone {
                name: fixed_name(r, at, 32)?,
                parent: w.i(0),
                flags: w.i(1),
                position: w.v3(8),
                rotation,
                quat: euler_to_quat(rotation),
                position_scale: w.v3(14),
                rotation_scale: w.v3(17),
                ..Bone::default()
            })
        })
        .collect()
}

/// The three attachment vectors are unused by the engine and studiomdl
/// rebuilds them, so only the origin is kept.
pub fn attachments(r: &Source, h: &GoldHeader) -> Result<Vec<Attachment>> {
    let (count, index) = h.span(ATTACHMENTS);
    table(r, 0, count, index, 88)?
        .into_iter()
        .map(|at| {
            let w = Words::read(r, at + 32, 5)?;
            let [x, y, z] = w.v3(2);
            Ok(Attachment {
                name: fixed_name(r, at, 32)?,
                bone: w.i(1),
                flags: w.i(0),
                local: [[1.0, 0.0, 0.0, x], [0.0, 1.0, 0.0, y], [0.0, 0.0, 1.0, z]],
            })
        })
        .collect()
}

/// GoldSrc has a single unnamed hitbox set.
pub fn hitbox_sets(r: &Source, h: &GoldHeader) -> Result<Vec<HitboxSet>> {
    let (count, index) = h.span(HITBOXES);
    let hitboxes = table(r, 0, count, index, 32)?
        .into_iter()
        .map(|at| {
            let w = Words::read(r, at, 8)?;
            Ok(Hitbox {
                bone: w.i(0),
                group: w.i(1),
                min: w.v3(2),
                max: w.v3(5),
                ..Hitbox::default()
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(vec![HitboxSet {
        name: "default".into(),
        hitboxes,
    }])
}
