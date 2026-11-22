use prybar_mdl::ir::SubModel;
use prybar_mdl::ModelIr;
use std::collections::HashSet;

/// Distinct (desc, partner, targets) flexes of a sub-model, i.e. the
/// shape keys an artist exported; every index is checked on the way.
pub fn flex_count(ir: &ModelIr, model: &SubModel) -> usize {
    let mut keys = HashSet::new();
    for mesh in &model.meshes {
        for f in &mesh.flexes {
            assert!(
                (f.desc as usize) < ir.flex_descs.len(),
                "flex desc {}",
                f.desc
            );
            assert!((f.partner as usize) < ir.flex_descs.len().max(1));
            for d in &f.deltas {
                assert!(i32::from(d.index) < mesh.vertex_count, "delta vertex");
                // float16 misread as fixed point (or the reverse) gives
                // either huge or all-zero shapes.
                assert!(
                    d.position.iter().all(|c| c.abs() < 64.0),
                    "{:?}",
                    d.position
                );
            }
            keys.insert((f.desc, f.partner, f.targets.map(f32::to_bits)));
        }
    }
    keys.len()
}

/// `time` lines of the `vertexanimation` section.
pub fn vta_frames(text: &str) -> usize {
    let (_, anim) = text.split_once("vertexanimation\n").unwrap();
    anim.lines().filter(|l| l.starts_with("time ")).count()
}

/// Bounding box of the reference mesh, the ragdoll's rest shape.
pub fn mesh_box(ir: &ModelIr) -> ([f32; 3], [f32; 3]) {
    let mut b = ([f32::MAX; 3], [f32::MIN; 3]);
    let verts = ir
        .body_parts
        .iter()
        .flat_map(|p| &p.models)
        .flat_map(|m| &m.vertices);
    for v in verts {
        for c in 0..3 {
            b.0[c] = b.0[c].min(v.position[c]);
            b.1[c] = b.1[c].max(v.position[c]);
        }
    }
    b
}

/// Share of collision points inside a box (plus a margin): a wrong space
/// or axis swap puts most of them outside.
pub fn inside(bbox: ([f32; 3], [f32; 3]), smd: &str) -> f32 {
    let (min, max) = bbox;
    let (_, tris) = smd.split_once("triangles\n").unwrap();
    let (mut n, mut ok) = (0usize, 0usize);
    for l in tris.lines().filter(|l| l.starts_with("  ")) {
        let v: Vec<f32> = l
            .split_whitespace()
            .skip(1)
            .take(3)
            .map(|x| x.parse().unwrap())
            .collect();
        let fits = (0..3).all(|c| v[c] >= min[c] - 2.0 && v[c] <= max[c] + 2.0);
        (n, ok) = (n + 1, ok + usize::from(fits));
    }
    ok as f32 / n.max(1) as f32
}
