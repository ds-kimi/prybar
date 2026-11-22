use crate::ir::{ConvexPiece, Vec3};
use crate::read::{offset, Source};
use crate::{Error, Result};

/// IVP works in meters with Y up; Source in inches with Z up.
const METERS_TO_INCHES: f32 = 1.0 / 0.0254;
const TRIANGLE: usize = 16;
const POINT: usize = 16;

pub struct Ledge {
    pub piece: ConvexPiece,
    pub next: usize,
    /// Start of the point array this ledge indexes into.
    pub points: usize,
}

/// `IVP_Compact_Ledge` at `at`, its triangles right after it.
pub fn ledge(r: &Source, at: usize, end: usize) -> Result<Ledge> {
    let point_rel: i32 = r.read(at)?;
    let client: i32 = r.read(at + 4)?;
    let n: i16 = r.read(at + 12)?;
    let bad = Error::Invalid {
        what: "phy ledge triangle count",
        value: n.into(),
    };
    let n = usize::try_from(n).map_err(|_| bad.clone())?;
    let next = at + TRIANGLE + n * TRIANGLE;
    if next > end {
        return Err(bad);
    }
    r.charge(n * 36)?;
    let points = offset(at, point_rel)?;
    let triangles = (0..n)
        .map(|t| triangle(r, at + TRIANGLE + t * TRIANGLE, points, end))
        .collect::<Result<_>>()?;
    Ok(Ledge {
        piece: ConvexPiece {
            // Forged client data must not overflow; -1 means no bone.
            bone: client.checked_sub(1).unwrap_or(-1),
            triangles,
        },
        next,
        points,
    })
}

/// Each of the three edges starts with the index of its first point.
fn triangle(r: &Source, at: usize, points: usize, end: usize) -> Result<[Vec3; 3]> {
    let mut tri = [[0.0; 3]; 3];
    for (k, v) in tri.iter_mut().enumerate() {
        let index: u16 = r.read(at + 4 + 4 * k)?;
        let p = points + usize::from(index) * POINT;
        if p + 12 > end {
            return Err(Error::Invalid {
                what: "phy point index",
                value: index.into(),
            });
        }
        let [x, y, z]: [f32; 3] = [r.read(p)?, r.read(p + 4)?, r.read(p + 8)?];
        *v = [x, z, -y].map(|c| c * METERS_TO_INCHES);
    }
    Ok(tri)
}
