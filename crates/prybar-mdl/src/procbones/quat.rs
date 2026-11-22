use crate::ir::{ProcBone, QuatTrigger};
use crate::read::{table, Source, Words};
use crate::Result;

/// `mstudioquatinterpinfo_t` size.
const TRIGGER_STRIDE: usize = 48;

pub fn quat_interp(r: &Source, at: usize) -> Result<ProcBone> {
    let triggers = table(r, at, r.read(at + 4)?, r.read(at + 8)?, TRIGGER_STRIDE)?
        .into_iter()
        .map(|t| trigger(r, t))
        .collect::<Result<_>>()?;
    Ok(ProcBone::QuatInterp {
        control: r.read(at)?,
        triggers,
    })
}

fn trigger(r: &Source, at: usize) -> Result<QuatTrigger> {
    let w = Words::read(r, at, TRIGGER_STRIDE / 4)?;
    let inv = w.f(0);
    Ok(QuatTrigger {
        tolerance: if inv != 0.0 { 1.0 / inv } else { 0.0 },
        trigger: [w.f(1), w.f(2), w.f(3), w.f(4)],
        position: w.v3(5),
        quat: [w.f(8), w.f(9), w.f(10), w.f(11)],
    })
}
