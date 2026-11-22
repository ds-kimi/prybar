use crate::read::offset;
use crate::Result;
use prybar_io::Reader;
use std::collections::HashMap;

/// Frame being decoded plus where each run-length stream stopped last
/// time. Frames are decoded in order, so resuming from the last run keeps
/// a whole anim linear instead of rewalking every stream from its start.
#[derive(Default)]
pub struct Step {
    pub frame: usize,
    /// stream start -> (run header offset, first frame of that run)
    runs: HashMap<usize, (usize, usize)>,
}

impl Step {
    /// Section changes restart frame numbering, so cursors are stale.
    pub fn reset(&mut self) {
        self.runs.clear();
    }
}

/// Raw per-axis values of an `mstudioanim_valueptr_t` at one frame; a
/// zero offset means the axis does not move.
pub fn axis_values(r: &Reader, ptr: usize, step: &mut Step) -> Result<[f32; 3]> {
    let mut out = [0.0; 3];
    for (i, v) in out.iter_mut().enumerate() {
        let rel: i16 = r.read(ptr + i * 2)?;
        if rel != 0 {
            *v = f32::from(run_value(r, offset(ptr, rel.into())?, step)?);
        }
    }
    Ok(out)
}

/// Walks the run-length stream: each run header is (valid, total), holding
/// `valid` stored values and repeating the last one up to `total` frames.
/// Every run covers at least one frame, so the walk is bounded by `frame`.
fn run_value(r: &Reader, start: usize, step: &mut Step) -> Result<i16> {
    let frame = step.frame;
    let (mut at, mut first) = match step.runs.get(&start) {
        Some(&(at, first)) if first <= frame => (at, first),
        _ => (start, 0),
    };
    loop {
        let valid = usize::from(r.read::<u8>(at)?);
        let total = usize::from(r.read::<u8>(at + 1)?);
        if total == 0 {
            return Ok(0);
        }
        let k = frame - first;
        if k < total {
            step.runs.insert(start, (at, first));
            let slot = if k < valid { k + 1 } else { valid };
            return Ok(r.read(at + slot * 2)?);
        }
        first += total;
        at += (valid + 1) * 2;
    }
}
