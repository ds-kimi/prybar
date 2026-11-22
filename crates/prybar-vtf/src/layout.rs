//! Where one mip of frame 0, face 0, slice 0 lives in the file.

use crate::error::{Error, Result};
use crate::header::Header;

/// Byte range and size of mip `level` (0 = full size). Mips are stored
/// smallest first, each holding every frame, face and slice.
pub fn mip_range(h: &Header, level: u8) -> Result<(std::ops::Range<usize>, u32, u32)> {
    if level >= h.mips.max(1) {
        return Err(Error::Mip(level));
    }
    let per_image = (h.frames as usize)
        .checked_mul(h.faces as usize)
        .ok_or(Error::Truncated)?;
    let mut at = h.data;
    for m in (level..h.mips.max(1)).rev() {
        let (w, hh) = (side(h.width, m), side(h.height, m));
        let one = h.format.size(w, hh).ok_or(Error::Truncated)?;
        let slices = side(h.depth, m) as usize;
        if m == level {
            let end = at.checked_add(one).ok_or(Error::Truncated)?;
            return Ok((at..end, w, hh));
        }
        let all = one
            .checked_mul(per_image)
            .and_then(|n| n.checked_mul(slices))
            .ok_or(Error::Truncated)?;
        at = at.checked_add(all).ok_or(Error::Truncated)?;
    }
    Err(Error::Mip(level))
}

fn side(full: u32, level: u8) -> u32 {
    full.checked_shr(level as u32).unwrap_or(0).max(1)
}
