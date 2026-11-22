use crate::{Error, Result};
use prybar_io::Reader;

pub const EOCD_SIG: u32 = 0x0605_4648;
/// Fixed record plus the largest comment a u16 length allows; scanning
/// further back would let a junk file cost a full-file walk.
const MAX_TAIL: usize = 22 + 0xFFFF;

#[derive(Debug, Clone, Copy)]
pub struct Eocd {
    pub count: usize,
    pub offset: usize,
}

/// The last signature wins, as in ZIP, because the comment may itself
/// contain bytes that look like a signature only before the real one.
pub fn find(data: &[u8]) -> Result<Eocd> {
    let r = Reader::new(data);
    let low = data.len().saturating_sub(MAX_TAIL);
    let last = data.len().checked_sub(22).ok_or(Error::NoDirectory)?;
    let at = (low..=last)
        .rev()
        .find(|&p| r.read::<u32>(p).ok() == Some(EOCD_SIG))
        .ok_or(Error::NoDirectory)?;
    Ok(Eocd {
        count: r.read::<u16>(at + 10)? as usize,
        offset: r.read::<u32>(at + 16)? as usize,
    })
}
