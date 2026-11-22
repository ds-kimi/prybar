use crate::ir::Movement;
use crate::read::{table, Source, Words};
use crate::Result;

/// `mstudiomovement_t` size.
const STRIDE: usize = 44;

pub fn movements(r: &Source, desc_at: usize, count: i32, index: i32) -> Result<Vec<Movement>> {
    table(r, desc_at, count, index, STRIDE)?
        .into_iter()
        .map(|at| {
            let w = Words::read(r, at, STRIDE / 4)?;
            Ok(Movement {
                end_frame: w.i(0),
                motion_flags: w.i(1),
                v0: w.f(2),
                v1: w.f(3),
                angle: w.f(4),
                vector: w.v3(5),
                position: w.v3(8),
            })
        })
        .collect()
}
