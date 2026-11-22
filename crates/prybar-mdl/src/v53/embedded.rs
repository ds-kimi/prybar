use crate::{Error, Result};
use prybar_io::Reader;

/// `(offset, size)` field pairs: offsets at 428.., sizes 16 bytes later.
const VTX: usize = 428;
const VVD: usize = 432;
const PHY: usize = 440;
const SIZES: usize = 16;

/// Companion files that v53 stores inside the MDL. Each is `None` when
/// its offset is 0; slices are bounds-checked against the MDL.
#[derive(Debug, Clone, Copy, Default)]
pub struct Embedded<'a> {
    pub vtx: Option<&'a [u8]>,
    pub vvd: Option<&'a [u8]>,
    pub phy: Option<&'a [u8]>,
}

pub fn embedded(data: &[u8]) -> Result<Embedded<'_>> {
    let r = Reader::new(data);
    let version: i32 = r.read(4)?;
    if version != 53 {
        return Err(Error::Unsupported { version });
    }
    Ok(Embedded {
        vtx: slice(&r, VTX)?,
        vvd: slice(&r, VVD)?,
        phy: slice(&r, PHY)?,
    })
}

fn slice<'a>(r: &Reader<'a>, field: usize) -> Result<Option<&'a [u8]>> {
    let at: i32 = r.read(field)?;
    let len: i32 = r.read(field + SIZES)?;
    if at == 0 {
        return Ok(None);
    }
    let bad = |value: i32| Error::Invalid {
        what: "embedded file offset or size",
        value: value.into(),
    };
    let at = usize::try_from(at).map_err(|_| bad(at))?;
    let len = usize::try_from(len).map_err(|_| bad(len))?;
    Ok(Some(r.bytes(at, len)?))
}
