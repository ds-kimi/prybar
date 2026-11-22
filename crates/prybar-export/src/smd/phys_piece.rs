use super::phys_math::face_normal;
use prybar_mdl::ir::Vec3;
use std::collections::HashMap;
use std::fmt::Write;

/// One convex piece. studiomdl welds collision vertices only when position
/// and normal both match: face normals would leave every face of a hull
/// loose (hundreds of parts, IVP failing or spinning on `$concave`), and
/// one shared normal per position would fuse hulls that touch. A normal
/// averaged within the piece keeps each hull whole and apart.
pub fn write(s: &mut String, bone: usize, tris: &[[Vec3; 3]]) {
    let normals = vertex_normals(tris);
    for tri in tris {
        s.push_str("phy\n");
        for p in tri {
            let [x, y, z] = *p;
            let [nx, ny, nz] = normals.get(&key(p)).copied().unwrap_or([0.0, 0.0, 1.0]);
            let _ = writeln!(
                s,
                "  {bone} {x:.6} {y:.6} {z:.6} {nx:.6} {ny:.6} {nz:.6} 0.000000 0.000000 1 {bone} 1.000000"
            );
        }
    }
}

fn vertex_normals(tris: &[[Vec3; 3]]) -> HashMap<[i64; 3], Vec3> {
    let mut sums: HashMap<[i64; 3], Vec3> = HashMap::new();
    for tri in tris {
        let n = face_normal(tri);
        for p in tri {
            let e = sums.entry(key(p)).or_default();
            *e = [0, 1, 2].map(|c| e[c] + n[c]);
        }
    }
    for n in sums.values_mut() {
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        *n = if len > 1e-12 {
            n.map(|c| c / len)
        } else {
            [0.0, 0.0, 1.0]
        };
    }
    sums
}

/// The text output rounds to 6 decimals, so positions equal on disk must
/// share a key.
fn key(p: &Vec3) -> [i64; 3] {
    p.map(|c| (f64::from(c) * 1e6).round() as i64)
}

#[cfg(test)]
mod tests {
    /// A corner shared by three faces of one hull gets one normal, so
    /// studiomdl welds the hull into a single convex part.
    #[test]
    fn shared_corner_has_one_normal() {
        let o = [0.0, 0.0, 0.0];
        let (x, y, z) = ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        let mut s = String::new();
        super::write(&mut s, 0, &[[o, y, x], [o, x, z], [o, z, y]]);
        let corners: Vec<&str> = s
            .lines()
            .filter(|l| l.starts_with("  0 0.000000 0.000000 0.000000"))
            .collect();
        assert_eq!(corners.len(), 3);
        assert!(corners.iter().all(|l| *l == corners[0]), "{s}");
    }
}
