//! Little-endian reads that fail instead of panicking past the end.

use crate::error::{Error, Result};

pub fn u16_at(b: &[u8], at: usize) -> Result<u32> {
    let s = b.get(at..at + 2).ok_or(Error::Truncated)?;
    Ok(u16::from_le_bytes([s[0], s[1]]) as u32)
}

pub fn u32_at(b: &[u8], at: usize) -> Result<u32> {
    let s = b.get(at..at + 4).ok_or(Error::Truncated)?;
    Ok(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

/// Checks the magic and major version 7; returns the minor (0 to 5).
pub fn minor_version(b: &[u8]) -> Result<u32> {
    if b.get(..4) != Some(b"VTF\0") {
        return Err(Error::BadMagic);
    }
    let minor = u32_at(b, 8)?;
    if u32_at(b, 4)? != 7 || minor > 5 {
        return Err(Error::BadVersion(minor));
    }
    Ok(minor)
}
