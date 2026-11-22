use super::header::MAX_NAME;
use crate::{Error, Result};
use prybar_io::Cursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub path: String,
    pub size: u64,
    pub crc: u32,
    /// Offset relative to the start of the data block.
    pub offset: u64,
}

/// Sizes are i64 on disk; negative or overflowing totals are rejected here
/// so later reads only need a bounds check.
pub(crate) fn parse_table(c: &mut Cursor) -> Result<Vec<Entry>> {
    let mut out = Vec::new();
    let mut offset = 0u64;
    while c.read::<u32>()? != 0 {
        let path = c.cstr(MAX_NAME)?.to_owned();
        let size = u64::try_from(c.read::<i64>()?).map_err(|_| Error::BadSize(path.clone()))?;
        let crc = c.read::<u32>()?;
        let next = offset.checked_add(size);
        let next = next.ok_or_else(|| Error::BadSize(path.clone()))?;
        out.push(Entry {
            path,
            size,
            crc,
            offset,
        });
        offset = next;
    }
    Ok(out)
}
