use super::shape::{int, Shape};
use crate::read::{table, Source};
use crate::{Error, Result};

const STRIP_IS_TRISTRIP: u8 = 2;

/// `groups` is the mesh's `(strip group count, offset)` pair.
pub fn mesh_triangles(
    r: &Source,
    at: usize,
    groups: (i32, i32),
    sh: &Shape,
) -> Result<Vec<[u16; 3]>> {
    let mut out = Vec::new();
    for g in table(r, at, groups.0, groups.1, sh.group_stride)? {
        group(r, g, sh, &mut out)?;
    }
    Ok(out)
}

fn group(r: &Source, at: usize, sh: &Shape, out: &mut Vec<[u16; 3]>) -> Result<()> {
    let [vc, vo, ic, io, sc, so, _] = sh.group;
    let verts = verts(r, at, sh, (int(r, at + vc, sh.narrow)?, r.read(at + vo)?))?;
    let index_at = table(r, at, int(r, at + ic, sh.narrow)?, r.read(at + io)?, 2)?;
    let indices: Vec<u16> = index_at
        .iter()
        .map(|&i| r.read(i))
        .collect::<prybar_io::Result<_>>()?;
    let [count_at, first_at, flags_at] = sh.strip;
    let strips = (int(r, at + sc, sh.narrow)?, r.read(at + so)?);
    for s in table(r, at, strips.0, strips.1, sh.strip_stride)? {
        let first = usize::try_from(int(r, s + first_at, sh.narrow)?).unwrap_or(usize::MAX);
        let count = usize::try_from(int(r, s + count_at, sh.narrow)?).unwrap_or(usize::MAX);
        let slice = first
            .checked_add(count)
            .and_then(|end| indices.get(first..end))
            .ok_or(Error::Invalid {
                what: "VTX strip index range starting at",
                value: first as i64,
            })?;
        // Strips may share index ranges, so emitted triangles are charged too.
        r.charge(slice.len() * 6)?;
        let strip = r.read::<u8>(s + flags_at)? & STRIP_IS_TRISTRIP != 0;
        emit(slice, strip, &verts, out)?;
    }
    Ok(())
}

/// Mesh-relative vertex ids of a group; static-prop groups store them bare.
fn verts(r: &Source, at: usize, sh: &Shape, span: (i32, i32)) -> Result<Vec<u16>> {
    let flags: u8 = r.read(at + sh.group[6])?;
    let (stride, id) = if sh.static_flag != 0 && flags & sh.static_flag != 0 {
        (2, 0)
    } else {
        (sh.vert_stride, sh.vert_id)
    };
    Ok(table(r, at, span.0, span.1, stride)?
        .into_iter()
        .map(|v| r.read::<u16>(v + id))
        .collect::<prybar_io::Result<Vec<u16>>>()?)
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
