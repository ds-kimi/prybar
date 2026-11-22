use glam::{Mat4, Vec3};

use crate::viewport::mesh::{Scene, STRIDE};

/// Writes the bind vertices of `bind` posed by `mats` into `out`. A
/// vertex without usable weights keeps its bind position.
pub fn skin(bind: &Scene, mats: &[Mat4], out: &mut Vec<f32>) {
    out.clear();
    out.extend_from_slice(&bind.vertices);
    for (v, w) in out
        .as_chunks_mut::<STRIDE>()
        .0
        .iter_mut()
        .zip(&bind.weights)
    {
        let (mut p, mut n, mut total) = (Vec3::ZERO, Vec3::ZERO, 0.0);
        let (bp, bn) = (Vec3::from_slice(&v[0..3]), Vec3::from_slice(&v[3..6]));
        for &(bone, weight) in w {
            let Some(m) = mats.get(usize::from(bone)) else {
                continue;
            };
            if weight > 0.0 && weight.is_finite() {
                p += m.transform_point3(bp) * weight;
                n += m.transform_vector3(bn) * weight;
                total += weight;
            }
        }
        if total > 1e-6 && p.is_finite() {
            v[0..3].copy_from_slice(&(p / total).to_array());
            v[3..6].copy_from_slice(&n.normalize_or_zero().to_array());
        }
    }
}

/// Bone segments (parent, child) for the posed skeleton overlay.
pub fn joints(world: &[Mat4], parents: &[Option<usize>]) -> Vec<[[f32; 3]; 2]> {
    let at = |i: usize| world[i].w_axis.truncate().to_array();
    let pairs = parents.iter().enumerate().take(world.len());
    pairs
        .filter_map(|(i, p)| p.filter(|&p| p < world.len()).map(|p| [at(p), at(i)]))
        .filter(|s| s.iter().flatten().all(|c| c.is_finite()))
        .collect()
}
