//! Material name -> VMT -> base texture, with where each was looked for.

use super::{normalize, vmt, Found, Sources};
use prybar_mdl::ir::ModelIr;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterialReport {
    /// Material name as stored in the model.
    pub name: String,
    /// `materials/...vmt` that was found, with its location.
    pub vmt: Option<(String, String)>,
    /// `$basetexture` value, normalized; `None` when no VMT or no key.
    pub base_texture: Option<String>,
    /// Location of the base texture VTF; `None` when it was not found.
    pub texture_location: Option<String>,
    /// Alpha test or translucency: the texture alpha cuts the surface.
    pub cutout: bool,
    /// VMT paths tried, for the "missing" message.
    pub tried: Vec<String>,
}

/// One report per material slot, in model order.
pub fn materials(ir: &ModelIr, sources: &Sources) -> Vec<MaterialReport> {
    ir.materials
        .iter()
        .map(|m| material(ir, &m.name, sources))
        .collect()
}

fn material(ir: &ModelIr, name: &str, sources: &Sources) -> MaterialReport {
    let (tried, vmt) = find_vmt(ir, name, sources);
    let text = vmt
        .as_ref()
        .map(|(_, f)| String::from_utf8_lossy(&f.bytes).into_owned());
    let base = text.as_deref().and_then(|t| follow(t, sources));
    let cutout = text.is_some_and(|t| vmt::flag(&t, &["$alphatest", "$translucent"]));
    let texture_location = base
        .as_ref()
        .and_then(|b| sources.locate(&format!("materials/{b}.vtf")));
    MaterialReport {
        name: name.to_owned(),
        vmt: vmt.map(|(p, f)| (p, f.location)),
        base_texture: base,
        texture_location,
        cutout,
        tried,
    }
}

/// Tries the name under every `$cdmaterials` folder, in model order.
fn find_vmt(ir: &ModelIr, name: &str, sources: &Sources) -> (Vec<String>, Option<(String, Found)>) {
    let dirs = if ir.material_dirs.is_empty() {
        vec![String::new()]
    } else {
        ir.material_dirs.clone()
    };
    let tried: Vec<String> = dirs
        .iter()
        .map(|d| format!("materials/{}.vmt", normalize(&format!("{d}/{name}"))))
        .collect();
    let vmt = tried
        .iter()
        .find_map(|p| sources.read(p).ok().flatten().map(|f| (p.clone(), f)));
    (tried, vmt)
}

/// `$basetexture`, following one patch `include` when the VMT has none.
fn follow(text: &str, sources: &Sources) -> Option<String> {
    match vmt::base_texture(text)? {
        Ok(base) => Some(normalize(&base)),
        Err(include) => {
            let found = sources
                .read(&format!(
                    "materials/{}.vmt",
                    normalize(&include).trim_start_matches("materials/")
                ))
                .ok()
                .flatten()?;
            match vmt::base_texture(&String::from_utf8_lossy(&found.bytes))? {
                Ok(base) => Some(normalize(&base)),
                Err(_) => None,
            }
        }
    }
}
