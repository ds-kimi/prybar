use crate::read::Source;
use crate::{Error, Result};

/// Order of raw vertices for the root LOD. Without fixups the file order is
/// already right; with them, only ranges tagged for LOD 0 or coarser are
/// kept, in table order, which is what the MDL vertex offsets index into.
pub fn root_order(r: &Source, fixups: &[usize], raw_len: usize) -> Result<Vec<usize>> {
    if fixups.is_empty() {
        return Ok((0..raw_len).collect());
    }
    let mut order = Vec::new();
    for &at in fixups {
        let lod: i32 = r.read(at)?;
        let start: i32 = r.read(at + 4)?;
        let count: i32 = r.read(at + 8)?;
        if lod < 0 {
            continue;
        }
        let range = checked_range(start, count, raw_len)?;
        r.charge(range.len())?;
        order.extend(range);
    }
    Ok(order)
}

fn checked_range(start: i32, count: i32, len: usize) -> Result<std::ops::Range<usize>> {
    let bad = Error::Invalid {
        what: "VVD fixup range starting at",
        value: start.into(),
    };
    let s = usize::try_from(start).map_err(|_| bad.clone())?;
    let n = usize::try_from(count).map_err(|_| bad.clone())?;
    let end = s.checked_add(n).filter(|&e| e <= len).ok_or(bad)?;
    Ok(s..end)
}
