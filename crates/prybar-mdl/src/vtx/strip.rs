use crate::read::{table, Source};
use crate::{Error, Result};

/// Packed `StripGroupHeader_t` as written for v44-v49 (no topology fields).
const GROUP_STRIDE: usize = 25;
const STRIP_STRIDE: usize = 27;
/// Packed `Vertex_t`; the mesh-relative VVD id sits at byte 4.
const VERT_STRIDE: usize = 9;
const STRIP_IS_TRISTRIP: u8 = 2;

pub fn mesh_triangles(r: &Source, at: usize) -> Result<Vec<[u16; 3]>> {
    let mut out = Vec::new();
    for g in table(r, at, r.read(at)?, r.read(at + 4)?, GROUP_STRIDE)? {
        group(r, g, &mut out)?;
    }
    Ok(out)
}

fn group(r: &Source, at: usize, out: &mut Vec<[u16; 3]>) -> Result<()> {
    let verts = table(r, at, r.read(at)?, r.read(at + 4)?, VERT_STRIDE)?
        .into_iter()
        .map(|v| r.read::<u16>(v + 4))
        .collect::<prybar_io::Result<Vec<u16>>>()?;
    let index_at = table(r, at, r.read(at + 8)?, r.read(at + 12)?, 2)?;
    let indices: Vec<u16> = index_at
        .iter()
        .map(|&i| r.read(i))
        .collect::<prybar_io::Result<_>>()?;
    for s in table(r, at, r.read(at + 16)?, r.read(at + 20)?, STRIP_STRIDE)? {
        let first = usize::try_from(r.read::<i32>(s + 4)?).unwrap_or(usize::MAX);
        let count = usize::try_from(r.read::<i32>(s)?).unwrap_or(usize::MAX);
        let slice = first
            .checked_add(count)
            .and_then(|end| indices.get(first..end))
            .ok_or(Error::Invalid {
                what: "VTX strip index range starting at",
                value: first as i64,
            })?;
        // Strips may share index ranges, so emitted triangles are charged too.
        r.charge(slice.len() * 6)?;
        let strip = r.read::<u8>(s + 18)? & STRIP_IS_TRISTRIP != 0;
        emit(slice, strip, &verts, out)?;
    }
    Ok(())
}

/// VTX winds clockwise and SMD counter-clockwise, so each triangle is
/// emitted as (a, c, b).
fn emit(idx: &[u16], tristrip: bool, verts: &[u16], out: &mut Vec<[u16; 3]>) -> Result<()> {
    let id = |i: u16| {
        verts.get(usize::from(i)).copied().ok_or(Error::Invalid {
            what: "VTX vertex index",
            value: i.into(),
        })
    };
    let tris: Vec<[u16; 3]> = if tristrip {
        idx.windows(3)
            .enumerate()
            .map(|(n, w)| {
                if n % 2 == 0 {
                    [w[0], w[1], w[2]]
                } else {
                    [w[1], w[0], w[2]]
                }
            })
            .filter(|t| t[0] != t[1] && t[1] != t[2] && t[0] != t[2])
            .collect()
    } else {
        idx.as_chunks::<3>().0.to_vec()
    };
    for [a, b, c] in tris {
        out.push([id(a)?, id(c)?, id(b)?]);
    }
    Ok(())
}
