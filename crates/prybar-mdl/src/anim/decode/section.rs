use crate::Result;
use prybar_io::Reader;

/// Fields of an animdesc that pick where a given frame is stored.
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub at: usize,
    pub frame_count: usize,
    pub block: i32,
    pub index: i32,
    pub section_index: i32,
    pub section_frames: i32,
}

/// Returns `(block, index, frame within that data)`. Long anims are split
/// into sections of `section_frames`; studiomdl stores the very last frame
/// in an extra trailing section, which the engine reads the same way.
pub fn locate(r: &Reader, l: &Layout, frame: usize) -> Result<(i32, i32, usize)> {
    let Ok(per) = usize::try_from(l.section_frames) else {
        return Ok((l.block, l.index, frame));
    };
    if per == 0 {
        return Ok((l.block, l.index, frame));
    }
    let (section, local) = if l.frame_count > per && frame + 1 == l.frame_count {
        (l.frame_count / per + 1, 0)
    } else {
        (frame / per, frame % per)
    };
    let table = crate::read::offset(l.at, l.section_index)?;
    let entry = section
        .checked_mul(8)
        .and_then(|o| o.checked_add(table))
        .ok_or(prybar_io::Error::OutOfBounds {
            offset: table,
            len: 8,
            size: r.len(),
        })?;
    Ok((r.read(entry)?, r.read(entry + 4)?, local))
}
