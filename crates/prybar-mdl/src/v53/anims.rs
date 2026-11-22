use crate::header::{fields::LOCAL_ANIMS, Header};
use crate::ir::Animation;
use crate::read::{name_at, table, Source, Words};
use crate::{Error, Result};

/// `mstudioanimdesc_t` size for v53 (no anim block fields).
const STRIDE: usize = 92;

/// Only the descriptor is read: v53 bone streams, movements and IK rules
/// use Respawn layouts that are not decoded yet, so frames stay empty the
/// same way they do for an anim whose `.ani` file is missing.
pub fn animations(r: &Source, hdr: &Header) -> Result<Vec<Animation>> {
    let (count, index) = hdr.span(LOCAL_ANIMS);
    table(r, 0, count, index, STRIDE)?
        .into_iter()
        .map(|at| animation(r, at))
        .collect()
}

fn animation(r: &Source, at: usize) -> Result<Animation> {
    let w = Words::read(r, at, STRIDE / 4)?;
    let frame_count = usize::try_from(w.i(4)).map_err(|_| Error::Invalid {
        what: "anim frame count",
        value: w.i(4).into(),
    })?;
    Ok(Animation {
        name: name_at(r, at, w.i(1))?,
        fps: w.f(2),
        flags: w.i(3),
        frame_count,
        ..Animation::default()
    })
}
