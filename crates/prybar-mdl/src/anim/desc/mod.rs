//! `mstudioanimdesc_t` table (v44-v49 layout) -> IR animations.

mod ikrule;
mod movement;
mod record;

use super::decode::AnimData;
use crate::header::{fields, Header};
use crate::ir::{Animation, Bone};
use crate::read::{table, Source};
use crate::Result;

/// `ani` is the companion `.ani` file, when the model has one.
pub fn animations(
    r: &Source,
    hdr: &Header,
    bones: &[Bone],
    ani: Option<&[u8]>,
) -> Result<Vec<Animation>> {
    let d = AnimData::new(r, ani.map(Source::new), blocks(r, hdr)?);
    let (count, index) = hdr.span(fields::LOCAL_ANIMS);
    table(r, 0, count, index, record::STRIDE)?
        .into_iter()
        .map(|at| record::animation(r, &d, at, bones))
        .collect()
}

fn blocks(r: &Source, hdr: &Header) -> Result<Vec<(i32, i32)>> {
    let (count, index) = hdr.span(fields::ANIM_BLOCKS);
    table(r, 0, count, index, 8)?
        .into_iter()
        .map(|at| Ok((r.read(at)?, r.read(at + 4)?)))
        .collect()
}
