use super::header::{fixed_name, GoldHeader, SKIN_FAMILIES, SKIN_INDEX, SKIN_REFS, TEXTURES};
use crate::ir::Texture;
use crate::read::{table, Source, Words};
use crate::{Error, Result};

/// The engine's own limit; anything larger is a forged size.
const MAX_SIDE: u32 = 4096;

/// `mstudiotexture_t`: 64-byte name, flags, width, height, data offset.
pub fn textures(r: &Source, h: &GoldHeader) -> Result<Vec<Texture>> {
    let (count, index) = h.span(TEXTURES);
    table(r, 0, count, index, 80)?
        .into_iter()
        .map(|at| texture(r, at))
        .collect()
}

/// Pixels are followed by a 256-colour RGB palette.
fn texture(r: &Source, at: usize) -> Result<Texture> {
    let w = Words::read(r, at + 64, 4)?;
    let side = |n: usize| {
        let v = w.i(n);
        u32::try_from(v)
            .ok()
            .filter(|&s| s <= MAX_SIDE)
            .ok_or(Error::Invalid {
                what: "texture size",
                value: v.into(),
            })
    };
    let (width, height) = (side(1)?, side(2)?);
    let len = (width * height) as usize;
    let data = usize::try_from(w.i(3)).map_err(|_| Error::Invalid {
        what: "texture offset",
        value: w.i(3).into(),
    })?;
    let pixels = r.bytes(data, len + 768)?;
    r.charge(len + 768)?;
    Ok(Texture {
        name: fixed_name(r, at, 64)?,
        flags: w.i(0),
        width,
        height,
        pixels: pixels[..len].to_vec(),
        palette: pixels[len..].to_vec(),
    })
}

/// `numskinfamilies` rows of `numskinref` shorts.
pub fn skins(r: &Source, h: &GoldHeader) -> Result<Vec<Vec<u16>>> {
    // Zero refs would make a zero-stride table that a forged family
    // count could turn into billions of empty rows.
    let Ok(n @ 1..) = usize::try_from(h.w.i(SKIN_REFS)) else {
        return Ok(Vec::new());
    };
    let rows = table(r, 0, h.w.i(SKIN_FAMILIES), h.w.i(SKIN_INDEX), n * 2)?;
    rows.into_iter().map(|at| Ok(r.array(at, n, n)?)).collect()
}
