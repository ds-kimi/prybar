//! Source and GoldSrc model (MDL) parsing into one version-neutral IR.
//!
//! Every version family parses into [`ModelIr`], so exporters never need
//! to know which MDL version they are looking at.

pub mod anim;
mod detect;
mod error;
mod flex;
mod goldsrc;
pub mod header;
mod inspect;
pub mod ir;
mod legacy;
mod mesh;
mod phy;
mod procbones;
mod read;
mod seq;
mod v44;
mod v53;
mod vtx;
mod vvd;

pub use detect::{detect, Family, Version};
pub use error::{Error, Result};
pub use goldsrc::{parse as parse_goldsrc, Companions};
pub use header::Header;
pub use inspect::{inspect, Summary};
pub use ir::ModelIr;
pub use mesh::load_geometry;
pub use phy::parse_phy;
pub use v44::parse as parse_v44;
pub use v53::{embedded, Embedded};
pub use vtx::{parse_vtx, Vtx};
pub use vvd::{parse_vvd, Vvd};

/// Parses any supported MDL into the IR.
pub fn parse(data: &[u8]) -> Result<ModelIr> {
    parse_with_ani(data, None)
}

/// Like [`parse`], plus the companion `.ani` file that holds the frames
/// of animations stored outside the MDL. Without it those animations keep
/// their metadata but have no frames.
pub fn parse_with_ani(data: &[u8], ani: Option<&[u8]>) -> Result<ModelIr> {
    let version = detect(data)?;
    match version.family {
        Family::Source44 | Family::Source52 | Family::Source53 => v44::parse(data, ani),
        Family::SourceLegacy => legacy::parse(data),
        Family::GoldSrc if matches!(version.number, 10 | 14) => {
            goldsrc::parse(data, Companions::default())
        }
        _ => Err(Error::Unsupported {
            version: version.number,
        }),
    }
}
