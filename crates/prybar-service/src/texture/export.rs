//! Textures written out as PNG, so a CLI user or an agent sees the same
//! picture the texture screen draws.

use std::path::{Path, PathBuf};

use serde::Serialize;

use super::{texture, Sources, Texture};
use crate::{Result, ServiceError};

#[derive(Debug, Clone, Serialize)]
pub struct TextureReport {
    /// The texture's engine path, or the file that was read.
    pub path: String,
    /// Folder or package it came from.
    pub location: String,
    pub format: &'static str,
    pub mips: u8,
    pub width: u32,
    pub height: u32,
    pub out: PathBuf,
}

/// Decodes a `.vtf` file on disk to `out` (PNG). `level` is clamped to
/// the last mip, so 0 is always full size.
pub fn export_file(vtf: &Path, level: u8, out: &Path) -> Result<TextureReport> {
    let bytes = std::fs::read(vtf).map_err(|e| ServiceError::io(vtf, e))?;
    let shown = vtf.display().to_string();
    let parsed = prybar_vtf::Vtf::parse(bytes).map_err(|e| ServiceError::invalid(&shown, e))?;
    let level = level.min(parsed.header.mips.saturating_sub(1));
    let image = parsed
        .decode(level)
        .map_err(|e| ServiceError::invalid(&shown, e))?;
    let tex = Texture {
        path: shown.clone(),
        location: shown,
        format: parsed.format_name(),
        mips: parsed.header.mips.max(1),
        image,
    };
    write(tex, out)
}

/// Finds `material` the way the engine would for `model` (its tree, its
/// package, then every configured game) and writes it to `out`.
pub fn export_material(
    model: &str,
    material: &str,
    level: u8,
    out: &Path,
) -> Result<TextureReport> {
    let games: Vec<PathBuf> = crate::tools::config_get()
        .map(|c| c.games.into_iter().map(|g| g.game_dir).collect())
        .unwrap_or_default();
    let tex = texture(&Sources::for_model(model, &games), material, level)?;
    write(tex, out)
}

fn write(tex: Texture, out: &Path) -> Result<TextureReport> {
    let (width, height) = (tex.image.width, tex.image.height);
    let img = image::RgbaImage::from_raw(width, height, tex.image.rgba).ok_or_else(|| {
        ServiceError::invalid(&tex.path, "decoded size does not match its pixels")
    })?;
    if let Some(dir) = out.parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir).map_err(|e| ServiceError::io(dir, e))?;
    }
    img.save_with_format(out, image::ImageFormat::Png)
        .map_err(|e| ServiceError::Tool(format!("{}: {e}", out.display())))?;
    Ok(TextureReport {
        path: tex.path,
        location: tex.location,
        format: tex.format,
        mips: tex.mips,
        width,
        height,
        out: out.to_path_buf(),
    })
}
