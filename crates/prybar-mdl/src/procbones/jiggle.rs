use crate::ir::Jiggle;
use crate::read::{Source, Words};
use crate::Result;

/// Only v48-v49 append the five boing floats; reading them on other
/// versions would run past the struct, and the QC writer reads missing
/// params as zero.
const FLOATS: usize = 34;
const BOING_FLOATS: usize = 5;

pub fn jiggle(r: &Source, at: usize, version: i32) -> Result<Jiggle> {
    let floats = match version {
        48 | 49 => FLOATS,
        _ => FLOATS - BOING_FLOATS,
    };
    let w = Words::read(r, at, floats + 1)?;
    Ok(Jiggle {
        flags: w.i(0),
        params: (1..=floats).map(|n| w.f(n)).collect(),
    })
}
