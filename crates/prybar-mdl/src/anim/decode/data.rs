use crate::read::{offset, Source};
use crate::{Error, Result};
use std::cell::Cell;

/// Decoded poses dwarf the compressed stream (a 4-byte RLE run covers up
/// to 255 frames, an absent bone costs nothing), so a forged frame count
/// could exhaust memory. The cap scales with the input: every GMod model
/// stays under 8 bone-frames per input byte, so 32 leaves room for real
/// models and bounds a tiny hostile file to a few MB.
const BONE_FRAMES_PER_BYTE: usize = 32;
const BONE_FRAMES_FLOOR: usize = 1 << 16;

/// Where animation bytes can live: the MDL itself, or an `.ani` file
/// split into blocks (`mstudioanimblock_t`, block 0 meaning the MDL).
pub struct AnimData<'s, 'a> {
    pub mdl: &'s Source<'a>,
    pub ani: Option<Source<'a>>,
    /// `(datastart, dataend)` per block, as stored in the MDL header.
    pub blocks: Vec<(i32, i32)>,
    left: Cell<usize>,
}

impl<'s, 'a> AnimData<'s, 'a> {
    pub fn new(mdl: &'s Source<'a>, ani: Option<Source<'a>>, blocks: Vec<(i32, i32)>) -> Self {
        let input = mdl.len() + ani.as_ref().map_or(0, |a| a.len());
        let cap = input
            .saturating_mul(BONE_FRAMES_PER_BYTE)
            .max(BONE_FRAMES_FLOOR);
        let left = Cell::new(cap);
        Self {
            mdl,
            ani,
            blocks,
            left,
        }
    }

    /// Resolves `(block, index)`; `None` when the block is in an `.ani`
    /// that was not supplied.
    pub fn locate(
        &self,
        block: i32,
        index: i32,
        desc_at: usize,
    ) -> Result<Option<(&Source<'a>, usize)>> {
        if block == 0 {
            return Ok(Some((self.mdl, offset(desc_at, index)?)));
        }
        let bad = Error::Invalid {
            what: "anim block",
            value: block.into(),
        };
        let slot = usize::try_from(block).map_err(|_| bad.clone())?;
        let &(start, _) = self.blocks.get(slot).ok_or(bad)?;
        let base = usize::try_from(start).map_err(|_| Error::Invalid {
            what: "anim block start",
            value: start.into(),
        })?;
        match &self.ani {
            Some(ani) => Ok(Some((ani, offset(base, index)?))),
            None => Ok(None),
        }
    }

    pub fn charge(&self, bone_frames: usize) -> Result<()> {
        let left = self
            .left
            .get()
            .checked_sub(bone_frames)
            .ok_or(Error::Invalid {
                what: "decoded bone-frames",
                value: bone_frames as i64,
            })?;
        self.left.set(left);
        Ok(())
    }
}
