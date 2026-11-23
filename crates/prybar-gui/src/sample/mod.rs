//! A synthetic model and package for previews and tests, so the app can
//! be shown without any game asset.

use std::io;
use std::path::{Path, PathBuf};

// The same generated model the parser tests use: tiny, valid, original.
#[path = "../../../prybar-mdl/tests/common/mod.rs"]
mod mdl;
#[path = "../../../prybar-vtf/tests/support/mod.rs"]
mod vtf;

pub struct Sample {
    pub model: PathBuf,
    pub package: PathBuf,
}

/// Where `--sample` writes its files; rewritten on every launch.
pub fn default_dir() -> PathBuf {
    std::env::temp_dir().join("prybar-gui-sample")
}

/// Writes `models/sample.mdl` (+ VVD, VTX and its material) under `dir/src` and packs
/// that folder as `dir/sample_dir.vpk`.
pub fn write(dir: &Path) -> io::Result<Sample> {
    let models = dir.join("src").join("models");
    std::fs::create_dir_all(&models)?;
    std::fs::write(models.join("sample.vvd"), spread(mdl::geometry::vvd()))?;
    std::fs::write(models.join("sample.dx90.vtx"), mdl::geometry::vtx())?;
    let model = models.join("sample.mdl");
    std::fs::write(&model, mdl::build())?;
    let materials = dir
        .join("src")
        .join("materials")
        .join("models")
        .join("test");
    std::fs::create_dir_all(&materials)?;
    let vmt = "VertexLitGeneric\n{\n\t$basetexture \"models/test/box\"\n}\n";
    std::fs::write(materials.join("box.vmt"), vmt)?;
    std::fs::write(materials.join("box.vtf"), texture())?;
    let package = dir.join("sample_dir.vpk");
    let opts = prybar_vpk::PackOptions {
        version: 2,
        chunk_size: None,
    };
    prybar_vpk::pack(&dir.join("src"), &package, &opts).map_err(io::Error::other)?;
    Ok(Sample { model, package })
}

/// A 64x64 BGR888 texture with every mip: rust tiles on grey, drawn
/// here so the sample needs no image file.
fn texture() -> Vec<u8> {
    let mut data = Vec::new();
    for side in [1u32, 2, 4, 8, 16, 32, 64] {
        for i in 0..side * side {
            let (x, y) = (i % side * 64 / side, i / side * 64 / side);
            let edge = x % 16 < 2 || y % 16 < 2;
            data.extend_from_slice(if edge {
                &[40, 70, 190]
            } else {
                &[120, 116, 112]
            });
        }
    }
    vtf::vtf(2, 3, (64, 64), 7, &data)
}

/// The parser fixture lays every vertex on the x axis (its tests pin
/// that); here y and z move off it so the 3D view has a surface to show.
fn spread(mut vvd: Vec<u8>) -> Vec<u8> {
    for i in 0..8usize {
        let at = 64 + i * 48 + 20;
        let (y, z) = ((i % 2) as f32 * 4.0, (i / 2) as f32);
        if let Some(b) = vvd.get_mut(at..at + 8) {
            b[..4].copy_from_slice(&y.to_le_bytes());
            b[4..].copy_from_slice(&z.to_le_bytes());
        }
    }
    vvd
}
