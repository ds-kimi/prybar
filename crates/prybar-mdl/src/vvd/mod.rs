//! VVD: the vertex pool shared by every LOD of a model.

mod fixup;
mod vertex;

use crate::ir::Vertex;
use crate::read::{table, Source};
use crate::{Error, Result};

const STRIDE: usize = 48;

#[derive(Debug, Clone, PartialEq)]
pub struct Vvd {
    pub checksum: i32,
    /// Root-LOD vertices with fixups applied, the order MDL offsets assume.
    pub vertices: Vec<Vertex>,
}

pub fn parse_vvd(data: &[u8]) -> Result<Vvd> {
    let r = Source::new(data);
    if r.bytes(0, 4)? != b"IDSV" {
        return Err(Error::Invalid {
            what: "VVD magic",
            value: i64::from(r.read::<i32>(0)?),
        });
    }
    let checksum = r.read(8)?;
    let lod0_count: i32 = r.read(16)?;
    let fixups = table(&r, 0, r.read(48)?, r.read(52)?, 12)?;
    let raw = table(&r, 0, raw_count(&r, lod0_count)?, r.read(56)?, STRIDE)?;
    let order = fixup::root_order(&r, &fixups, raw.len())?;
    let vertices = order
        .into_iter()
        .map(|i| vertex::read(&r, raw[i]))
        .collect::<Result<_>>()?;
    Ok(Vvd { checksum, vertices })
}

/// Fixups may pull vertices in any order, so the raw pool size comes from
/// the gap before the tangent block rather than from the LOD 0 count.
fn raw_count(r: &Source, lod0_count: i32) -> Result<i32> {
    let vertex_start: i32 = r.read(56)?;
    let tangent_start: i32 = r.read(60)?;
    let gap = i64::from(tangent_start) - i64::from(vertex_start);
    if gap <= 0 {
        return Ok(lod0_count);
    }
    Ok(i32::try_from(gap / STRIDE as i64).unwrap_or(lod0_count))
}
