//! Header and image-data location for VTF 7.0 to 7.5.

use crate::error::{Error, Result};
use crate::format::Format;
use crate::read::{u16_at, u32_at};

const ENVMAP: u32 = 0x4000;
/// Larger images would need gigabytes once decoded; real textures stop
/// at 4096 and the engine itself refuses more than 8192.
pub const MAX_SIDE: u32 = 8192;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub minor: u32,
    pub width: u32,
    pub height: u32,
    pub flags: u32,
    pub frames: u32,
    pub faces: u32,
    pub depth: u32,
    pub mips: u8,
    pub format_id: i32,
    pub format: Format,
    /// Offset of the high-resolution data (smallest mip first).
    pub data: usize,
}

impl Header {
    pub fn parse(b: &[u8]) -> Result<Header> {
        let minor = crate::read::minor_version(b)?;
        let (width, height) = (u16_at(b, 16)?, u16_at(b, 18)?);
        if width == 0 || height == 0 || width > MAX_SIDE || height > MAX_SIDE {
            return Err(Error::Size(width, height));
        }
        let format_id = u32_at(b, 52)? as i32;
        let format = Format::from_id(format_id).ok_or(Error::Format(format_id))?;
        let flags = u32_at(b, 20)?;
        let faces = faces(flags, minor, u16_at(b, 26)?);
        let depth = if minor >= 2 { u16_at(b, 63)?.max(1) } else { 1 };
        let data = data_offset(b, minor)?;
        let (frames, mips) = (u16_at(b, 24)?.max(1), *b.get(56).ok_or(Error::Truncated)?);
        #[rustfmt::skip]
        let h = Header { minor, width, height, flags, frames, faces, depth, mips, format_id, format, data };
        Ok(h)
    }
}

/// Pre-7.5 cubemaps carry a seventh sphere-map face unless the
/// first-frame field says otherwise; skipping it shifts every mip.
fn faces(flags: u32, minor: u32, first_frame: u32) -> u32 {
    match flags & ENVMAP != 0 {
        true if minor < 5 && first_frame != 0xffff => 7,
        true => 6,
        false => 1,
    }
}

/// Before 7.3 the low-res thumbnail sits right after the header and the
/// image follows it; from 7.3 a resource table points at the image.
fn data_offset(b: &[u8], minor: u32) -> Result<usize> {
    let header_size = u32_at(b, 12)? as usize;
    if minor >= 3 {
        let count = u32_at(b, 68)?.min(32) as usize;
        for i in 0..count {
            let at = 80 + i * 8;
            if b.get(at..at + 3) == Some(&[0x30, 0, 0]) {
                return Ok(u32_at(b, at + 4)? as usize);
            }
        }
        return Err(Error::Truncated);
    }
    let low_id = u32_at(b, 57)? as i32;
    let (lw, lh) = (
        *b.get(61).ok_or(Error::Truncated)?,
        *b.get(62).ok_or(Error::Truncated)?,
    );
    let low = match (lw, lh, Format::from_id(low_id)) {
        (0, _, _) | (_, 0, _) | (_, _, None) => 0,
        (w, h, Some(f)) => f.size(w as u32, h as u32).unwrap_or(0),
    };
    Ok(header_size + low)
}
