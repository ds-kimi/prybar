use crate::read::{Source, Words};
use crate::{Error, Result};

/// `studiohdr_t` of v10 is 61 words; v14 appends more after them.
const WORDS: usize = 61;

pub const EYE: usize = 19;
pub const HULL_MIN: usize = 22;
pub const HULL_MAX: usize = 25;
pub const VIEW_MIN: usize = 28;
pub const VIEW_MAX: usize = 31;
pub const FLAGS: usize = 34;
pub const BONES: usize = 35;
pub const BONE_CONTROLLERS: usize = 37;
pub const HITBOXES: usize = 39;
pub const SEQS: usize = 41;
pub const SEQ_GROUPS: usize = 43;
pub const TEXTURES: usize = 45;
pub const SKIN_REFS: usize = 48;
pub const SKIN_FAMILIES: usize = 49;
pub const SKIN_INDEX: usize = 50;
pub const BODY_PARTS: usize = 51;
pub const ATTACHMENTS: usize = 53;

/// GoldSrc offsets are absolute, so the header is just its words.
pub struct GoldHeader {
    pub version: i32,
    pub name: String,
    pub w: Words,
}

impl GoldHeader {
    pub fn parse(r: &Source) -> Result<Self> {
        Ok(Self {
            version: r.read(4)?,
            name: fixed_name(r, 8, 64)?,
            w: Words::read(r, 0, WORDS)?,
        })
    }

    /// `(count, index)` pair starting at a slot.
    pub fn span(&self, slot: usize) -> (i32, i32) {
        (self.w.i(slot), self.w.i(slot + 1))
    }
}

/// GoldSrc stores names in fixed char arrays, NUL-padded.
pub fn fixed_name(r: &Source, at: usize, len: usize) -> Result<String> {
    let raw = r.bytes(at, len).map_err(Error::from)?;
    let end = raw.iter().position(|&b| b == 0).unwrap_or(len);
    Ok(String::from_utf8_lossy(&raw[..end]).into_owned())
}
