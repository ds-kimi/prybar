use super::ledge::ledge;
use crate::ir::PhysSolid;
use crate::read::Source;
use crate::{Error, Result};

/// `compactsurfaceheader_t` after its size field; files from before
/// `VPHY` existed start directly with the IVP surface.
const VPHY_HEADER: usize = 28;
/// `IVP_Compact_Surface`; ledges follow it.
const SURFACE_HEADER: usize = 48;
/// `modelType` of an IVP compact surface, the only one studiomdl writes.
const IVP_COMPACT: i16 = 0;

/// Parses the solid whose size field is at `at`; returns it and the
/// offset of the next solid.
pub fn solid(r: &Source, at: usize) -> Result<(PhysSolid, usize)> {
    let size: i32 = r.read(at)?;
    let body = at + 4;
    let end = usize::try_from(size)
        .ok()
        .and_then(|s| body.checked_add(s))
        .filter(|&e| e <= r.len())
        .ok_or(Error::Invalid {
            what: "phy solid size",
            value: size.into(),
        })?;
    let surface = surface_start(r, body)?;
    Ok((
        PhysSolid {
            pieces: ledges(r, surface, end)?,
        },
        end,
    ))
}

fn surface_start(r: &Source, body: usize) -> Result<usize> {
    if r.bytes(body, 4)? != b"VPHY" {
        return Ok(body);
    }
    let kind: i16 = r.read(body + 6)?;
    if kind != IVP_COMPACT {
        return Err(Error::Invalid {
            what: "phy model type",
            value: kind.into(),
        });
    }
    Ok(body + VPHY_HEADER)
}

/// Ledges sit back to back after the surface header and end where the
/// first point array begins, which bounds the walk without trusting the
/// ledge tree.
fn ledges(r: &Source, surface: usize, end: usize) -> Result<Vec<crate::ir::ConvexPiece>> {
    let mut at = surface + SURFACE_HEADER;
    let mut points = end;
    let mut pieces = Vec::new();
    while at + 16 <= points {
        let l = ledge(r, at, end)?;
        points = points.min(l.points);
        pieces.push(l.piece);
        at = l.next;
    }
    Ok(pieces)
}
