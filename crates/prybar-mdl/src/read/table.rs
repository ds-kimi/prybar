use super::Source;
use crate::{Error, Result};

/// Longest name accepted; real names are far shorter, the cap only stops
/// a hostile offset from turning into a huge allocation.
const NAME_MAX: usize = 1024;

/// Source offsets are signed and relative to the struct that holds them.
pub fn offset(base: usize, rel: i32) -> Result<usize> {
    let abs = base as i64 + i64::from(rel);
    usize::try_from(abs).map_err(|_| Error::Invalid {
        what: "offset",
        value: abs,
    })
}

/// Validates a `(count, index)` pair and returns the start of each record,
/// checking the whole range up front so callers can read without fear.
pub fn table(r: &Source, base: usize, count: i32, rel: i32, stride: usize) -> Result<Vec<usize>> {
    let n = usize::try_from(count).map_err(|_| Error::Invalid {
        what: "count",
        value: count.into(),
    })?;
    if n == 0 {
        return Ok(Vec::new());
    }
    let start = offset(base, rel)?;
    let len = n.checked_mul(stride).ok_or(Error::Invalid {
        what: "count",
        value: count.into(),
    })?;
    r.bytes(start, len)?;
    r.charge(len)?;
    Ok((0..n).map(|i| start + i * stride).collect())
}

/// Reads a NUL-terminated name; offset 0 means "no name" in every version.
/// Invalid UTF-8 is replaced rather than rejected because some community
/// models carry Latin-1 names that studiomdl accepted.
pub fn name_at(r: &Source, base: usize, rel: i32) -> Result<String> {
    if rel == 0 {
        return Ok(String::new());
    }
    let start = offset(base, rel)?;
    let avail = r.len().saturating_sub(start).min(NAME_MAX);
    let window = r.bytes(start, avail)?;
    let end = window.iter().position(|&b| b == 0).ok_or(Error::Invalid {
        what: "unterminated name at",
        value: start as i64,
    })?;
    r.charge(end + 1)?;
    Ok(String::from_utf8_lossy(&window[..end]).into_owned())
}
