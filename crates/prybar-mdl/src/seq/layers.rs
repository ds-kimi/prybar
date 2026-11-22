use crate::ir::{AutoLayer, Event, IkLock};
use crate::read::{name_at, table, Source, Words};
use crate::Result;

/// `mstudioevent_t`: cycle, event, type, options[64], name index.
pub fn events(r: &Source, seq: usize, count: i32, index: i32) -> Result<Vec<Event>> {
    table(r, seq, count, index, 80)?
        .into_iter()
        .map(|at| {
            let w = Words::read(r, at, 20)?;
            let raw = r.bytes(at + 12, 64)?;
            let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
            Ok(Event {
                cycle: w.f(0),
                event: w.i(1),
                kind: w.i(2),
                options: String::from_utf8_lossy(&raw[..end]).into_owned(),
                name: name_at(r, at, w.i(19))?,
            })
        })
        .collect()
}

/// `mstudioautolayer_t` (24 bytes).
pub fn auto_layers(r: &Source, seq: usize, count: i32, index: i32) -> Result<Vec<AutoLayer>> {
    table(r, seq, count, index, 24)?
        .into_iter()
        .map(|at| {
            let w = Words::read(r, at, 6)?;
            Ok(AutoLayer {
                sequence: r.read(at)?,
                pose: r.read(at + 2)?,
                flags: w.i(1),
                ramp: [w.f(2), w.f(3), w.f(4), w.f(5)],
            })
        })
        .collect()
}

/// `mstudioiklock_t` (32 bytes).
pub fn ik_locks(r: &Source, seq: usize, count: i32, index: i32) -> Result<Vec<IkLock>> {
    table(r, seq, count, index, 32)?
        .into_iter()
        .map(|at| {
            let w = Words::read(r, at, 4)?;
            Ok(IkLock {
                chain: w.i(0),
                pos_weight: w.f(1),
                local_q_weight: w.f(2),
                flags: w.i(3),
            })
        })
        .collect()
}
