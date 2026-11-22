//! A synthetic model and package for previews and tests, so the app can
//! be shown without any game asset.

use std::io;
use std::path::{Path, PathBuf};

// The same generated model the parser tests use: tiny, valid, original.
#[path = "../../../prybar-mdl/tests/common/mod.rs"]
mod mdl;

pub struct Sample {
    pub model: PathBuf,
    pub package: PathBuf,
}

/// Where `--sample` writes its files; rewritten on every launch.
pub fn default_dir() -> PathBuf {
    std::env::temp_dir().join("prybar-gui-sample")
}

/// Writes `models/sample.mdl` (+ VVD and VTX) under `dir/src` and packs
/// that folder as `dir/sample_dir.vpk`.
pub fn write(dir: &Path) -> io::Result<Sample> {
    let models = dir.join("src").join("models");
    std::fs::create_dir_all(&models)?;
    std::fs::write(models.join("sample.vvd"), mdl::geometry::vvd())?;
    std::fs::write(models.join("sample.dx90.vtx"), mdl::geometry::vtx())?;
    let model = models.join("sample.mdl");
    std::fs::write(&model, mdl::build())?;
    let package = dir.join("sample_dir.vpk");
    let opts = prybar_vpk::PackOptions {
        version: 2,
        chunk_size: None,
    };
    prybar_vpk::pack(&dir.join("src"), &package, &opts).map_err(io::Error::other)?;
    Ok(Sample { model, package })
}
