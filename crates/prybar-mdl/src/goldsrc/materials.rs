use super::header::GoldHeader;
use super::skin;
use crate::ir::{Material, Texture};
use crate::read::Source;
use crate::{ModelIr, Result};

/// Textures and skins come from the `T.mdl` when the model has none.
pub fn load(r: &Source, h: &GoldHeader, tex: Option<&[u8]>, ir: &mut ModelIr) -> Result<()> {
    let (mut textures, mut skins) = (skin::textures(r, h)?, skin::skins(r, h)?);
    if let (true, Some(t)) = (textures.is_empty(), tex) {
        let tr = Source::new(t);
        let th = GoldHeader::parse(&tr)?;
        (textures, skins) = (skin::textures(&tr, &th)?, skin::skins(&tr, &th)?);
    }
    ir.materials = textures
        .iter()
        .map(|t| Material {
            name: t.name.clone(),
            flags: t.flags,
        })
        .collect();
    (ir.textures, ir.skins) = (textures, skins);
    Ok(())
}

/// Size of the texture behind each skin slot of the default family.
pub fn slot_dims(ir: &ModelIr) -> Vec<(f32, f32)> {
    let size = |t: &Texture| (t.width.max(1) as f32, t.height.max(1) as f32);
    match ir.skins.first() {
        Some(row) => row
            .iter()
            .map(|&m| ir.textures.get(usize::from(m)).map_or((1.0, 1.0), size))
            .collect(),
        None => ir.textures.iter().map(size).collect(),
    }
}
