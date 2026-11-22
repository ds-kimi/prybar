//! Finds a model's materials and their base textures, and decodes them.
//!
//! The engine searches the model's folder tree, the package it came from,
//! then every mounted game; previews follow the same order so what shows
//! is what the game would draw.

mod lookup;
mod roots;
mod sources;
mod vmt;

pub use lookup::{materials, MaterialReport};
pub use sources::{Found, Sources};
pub use vmt::base_texture;

use crate::{Result, ServiceError};

/// Decodes `vtf` at mip `level` and keeps the file facts for display.
#[derive(Debug, Clone)]
pub struct Texture {
    pub path: String,
    pub location: String,
    pub format: &'static str,
    pub mips: u8,
    pub image: prybar_vtf::Image,
}

/// Reads and decodes `materials/<name>.vtf` from the first source holding it.
pub fn texture(sources: &Sources, name: &str, level: u8) -> Result<Texture> {
    let path = format!("materials/{}.vtf", normalize(name));
    let found = sources
        .read(&path)?
        .ok_or_else(|| ServiceError::NotFound(path.clone()))?;
    let vtf = prybar_vtf::Vtf::parse(found.bytes).map_err(|e| ServiceError::invalid(&path, e))?;
    let level = level.min(vtf.header.mips.saturating_sub(1));
    let image = vtf
        .decode(level)
        .map_err(|e| ServiceError::invalid(&path, e))?;
    Ok(Texture {
        path,
        location: found.location,
        format: vtf.format_name(),
        mips: vtf.header.mips.max(1),
        image,
    })
}

/// Engine paths are case-insensitive with either slash; lookups use one form.
pub(crate) fn normalize(name: &str) -> String {
    let p = name.trim().replace('\\', "/").to_ascii_lowercase();
    // `$cdmaterials` ends with a slash and names are joined after it.
    let p = p
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("/");
    let p = p.as_str();
    p.strip_suffix(".vtf")
        .or(p.strip_suffix(".vmt"))
        .unwrap_or(p)
        .to_owned()
}
