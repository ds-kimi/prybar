use super::anim::frames;
use super::header::{fixed_name, GoldHeader, SEQS};
use crate::ir::{Animation, Bone, Event, Sequence};
use crate::read::{table, Source, Words};
use crate::{Error, Result};

/// Decoded sequences and the animations they own, one per blend.
pub type Sequences = (Vec<Sequence>, Vec<Animation>);

/// studiomdl's own blend limit.
const MAX_BLENDS: i32 = 16;

/// `mstudioseqdesc_t`: 32-byte label then 36 words. Group 0 is stored in
/// the model; group `n` lives in `groups[n - 1]` (the `NN.mdl` files)
/// and its animations stay frameless when that file is not supplied.
pub fn sequences(
    r: &Source,
    h: &GoldHeader,
    bones: &[Bone],
    groups: &[Source],
) -> Result<Sequences> {
    let (count, index) = h.span(SEQS);
    let (mut seqs, mut anims) = (Vec::new(), Vec::new());
    for at in table(r, 0, count, index, 176)? {
        let w = Words::read(r, at + 32, 36)?;
        let mut seq = sequence(r, at, &w)?;
        let src = match w.i(31) {
            0 => Some(r),
            g => usize::try_from(g)
                .ok()
                .and_then(|g| groups.get(g.checked_sub(1)?)),
        };
        for anim in blends(src, &w, bones, &seq.name)? {
            let id = i16::try_from(anims.len()).map_err(|_| Error::Invalid {
                what: "animation count",
                value: anims.len() as i64,
            })?;
            seq.anims.push(id);
            anims.push(anim);
        }
        seqs.push(seq);
    }
    Ok((seqs, anims))
}

/// Blend `b` stores its per-bone anim structs right after blend `b - 1`.
fn blends(src: Option<&Source>, w: &Words, bones: &[Bone], name: &str) -> Result<Vec<Animation>> {
    let count = usize::try_from(w.i(6)).unwrap_or(0);
    let base = usize::try_from(w.i(23)).unwrap_or(usize::MAX);
    (0..w.i(22).clamp(1, MAX_BLENDS) as usize)
        .map(|b| {
            let frames = match src {
                Some(g) => frames(g, base.saturating_add(b * bones.len() * 12), bones, count)?,
                None => Vec::new(),
            };
            Ok(Animation {
                name: if b == 0 {
                    name.into()
                } else {
                    format!("{name}_blend{b}")
                },
                fps: w.f(0),
                flags: w.i(1),
                frame_count: count,
                frames,
                ..Animation::default()
            })
        })
        .collect()
}

fn sequence(r: &Source, at: usize, w: &Words) -> Result<Sequence> {
    Ok(Sequence {
        name: fixed_name(r, at, 32)?,
        flags: w.i(1),
        activity: w.i(2),
        activity_weight: w.i(3),
        events: events(r, w)?,
        bb_min: w.v3(16),
        bb_max: w.v3(19),
        group_size: [w.i(22).clamp(1, MAX_BLENDS), 1],
        entry_node: w.i(32),
        exit_node: w.i(33),
        node_flags: w.i(34),
        next_seq: w.i(35),
        ..Sequence::default()
    })
}

/// `mstudioevent_t`: frame, event, type, 64-byte options. Frames become
/// cycles because that is how the IR stores event timing.
fn events(r: &Source, w: &Words) -> Result<Vec<Event>> {
    let last = w.i(6).saturating_sub(1).max(1) as f32;
    table(r, 0, w.i(4), w.i(5), 76)?
        .into_iter()
        .map(|at| {
            let e = Words::read(r, at, 3)?;
            Ok(Event {
                cycle: e.i(0) as f32 / last,
                event: e.i(1),
                kind: e.i(2),
                options: fixed_name(r, at + 12, 64)?,
                ..Event::default()
            })
        })
        .collect()
}
