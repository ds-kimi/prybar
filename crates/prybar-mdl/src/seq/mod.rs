//! `mstudioseqdesc_t` table (v44-v53 layout) and pose parameters.

mod layers;
mod lists;
mod record;

pub(crate) use layers::ik_locks;

use crate::header::{fields, Header};
use crate::ir::{PoseParam, Sequence};
use crate::read::{name_at, table, Source, Words};
use crate::Result;

pub fn sequences(r: &Source, hdr: &Header, bone_count: usize) -> Result<Vec<Sequence>> {
    let (count, index) = hdr.span(fields::LOCAL_SEQS);
    // v53 pads the struct with five more unused ints.
    let stride = record::STRIDE + if hdr.version == 53 { 20 } else { 0 };
    table(r, 0, count, index, stride)?
        .into_iter()
        .map(|at| record::sequence(r, at, bone_count, hdr.version))
        .collect()
}

/// `mstudioposeparamdesc_t` (20 bytes).
pub fn pose_params(r: &Source, hdr: &Header) -> Result<Vec<PoseParam>> {
    let (count, index) = hdr.span(fields::POSE_PARAMS);
    table(r, 0, count, index, 20)?
        .into_iter()
        .map(|at| {
            let w = Words::read(r, at, 5)?;
            Ok(PoseParam {
                name: name_at(r, at, w.i(0))?,
                flags: w.i(1),
                start: w.f(2),
                end: w.f(3),
                loop_range: w.f(4),
            })
        })
        .collect()
}
