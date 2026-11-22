mod extra;
mod materials;

use crate::header::Header;
use crate::read::Source;
use crate::{ModelIr, Result};

pub fn fill(r: &Source, hdr: &Header, ir: &mut ModelIr) -> Result<()> {
    ir.materials = materials::materials(r, hdr)?;
    ir.material_dirs = materials::material_dirs(r, hdr)?;
    ir.skins = materials::skins(r, hdr)?;
    ir.include_models = extra::include_models(r, hdr)?;
    ir.key_values = extra::key_values(r, hdr)?;
    ir.surface_prop = extra::surface_prop(r, hdr)?;
    Ok(())
}
