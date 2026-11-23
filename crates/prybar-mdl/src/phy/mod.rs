//! `.phy` collision files: IVP compact surfaces (one per solid, each a set
//! of convex ledges) followed by a key-value text section.

mod ledge;
mod solid;
mod text;

use crate::ir::Physics;
use crate::read::Source;
use crate::{Error, Result};
use prybar_io::Reader;

/// `phyheader_t` is 16 bytes; a larger size field leaves room for growth.
const MIN_HEADER: usize = 16;

pub fn parse_phy(data: &[u8]) -> Result<Physics> {
    let r = Source::with_reader(probe_order(data));
    let size: i32 = r.read(0)?;
    let count: i32 = r.read(8)?;
    let mut at = usize::try_from(size)
        .ok()
        .filter(|&s| s >= MIN_HEADER)
        .ok_or(Error::Invalid {
            what: "phy header size",
            value: size.into(),
        })?;
    let count = solid_count(&r, at, count)?;
    let mut solids = Vec::with_capacity(count);
    for _ in 0..count {
        let (solid, next) = solid::solid(&r, at)?;
        solids.push(solid);
        at = next;
    }
    let text = text::section(&r, at)?;
    Ok(Physics {
        checksum: r.read(12)?,
        solids,
        blocks: text::blocks(&text),
        text,
    })
}

/// Console ports (Postal III and other Xbox 360 / PS3 builds) write the
/// whole binary part big-endian. The header size is always 16 in practice,
/// so a little-endian size that is not plausible but reads 16 swapped
/// decides the order; anything else stays little-endian as before.
fn probe_order(data: &[u8]) -> Reader<'_> {
    let le = Reader::new(data);
    let be = Reader::big_endian(data);
    let swapped = le.read::<i32>(0).is_ok_and(|s| s > MIN_HEADER as i32)
        && be.read::<i32>(0).is_ok_and(|s| s == MIN_HEADER as i32);
    if swapped {
        be
    } else {
        le
    }
}

/// Each solid takes at least its 4-byte size field, which caps the count
/// before anything is allocated.
fn solid_count(r: &Source, at: usize, count: i32) -> Result<usize> {
    let max = r.len().saturating_sub(at) / 4;
    usize::try_from(count)
        .ok()
        .filter(|&n| n <= max)
        .ok_or(Error::Invalid {
            what: "phy solid count",
            value: count.into(),
        })
}
