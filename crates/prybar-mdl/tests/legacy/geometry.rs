//! MDL 2531 inline vertices in their three formats, and VTX v107 strips.

use super::inline::{model, skinned};
use super::vtx107::vtx107;
use prybar_mdl::{load_triangles, parse_vtx};

#[test]
fn skinned_vertices_keep_up_to_four_weights() {
    let ir = model(
        0,
        44,
        &[
            skinned([100, 55, 0, 3], 5.0),
            skinned([100, 100, 50, 4], 6.0),
        ],
    );
    assert!(
        !ir.notes.iter().any(|n| n.starts_with("meshes")),
        "{:?}",
        ir.notes
    );
    let m = &ir.body_parts[0].models[0];
    assert_eq!((m.meshes.len(), m.meshes[0].vertex_count), (1, 2));
    let [a, b] = [&m.vertices[0], &m.vertices[1]];
    assert_eq!(
        (a.position, a.normal, a.uv),
        ([5.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.25, 0.75])
    );
    assert_eq!(a.weights, vec![(1, 100.0 / 255.0), (2, 55.0 / 255.0)]);
    let bones: Vec<u8> = b.weights.iter().map(|w| w.0).collect();
    assert_eq!((bones, b.weights[3].1), (vec![1, 2, 3, 0], 5.0 / 255.0));
}

#[test]
fn packed_vertices_dequantize_into_the_model_bounds() {
    let fine_top = (5313u16 * 12).to_le_bytes();
    let wide = [1, 0, 2, 0, 3, 0, fine_top[0], fine_top[1], 255, 255, 0, 0].to_vec();
    let v = &model(1, 12, &[wide]).body_parts[0].models[0].vertices[0];
    assert_eq!(
        (v.position, v.normal, v.uv),
        ([2.0, 4.0, 6.0], [0.0, 0.0, 1.0], [1.0, 0.0])
    );
    assert_eq!(v.weights, vec![(0, 1.0)]);
    let small = vec![255, 0, 0, 0, 0, 0, 255, 255];
    let v = &model(2, 8, &[small]).body_parts[0].models[0].vertices[0];
    assert_eq!((v.position, v.normal), ([2.0, 0.0, 0.0], [0.0, 0.0, -1.0]));
    let last = vec![0, 0, 0, 201, 0, 0, 0, 0];
    let v = &model(2, 8, &[last]).body_parts[0].models[0].vertices[0];
    assert_eq!(v.normal, [0.0, 0.0, 1.0]);
}

#[test]
fn bad_normal_drops_the_meshes_but_keeps_the_model() {
    let ir = model(2, 8, &[vec![0, 0, 0, 202, 0, 0, 0, 0]]);
    let m = &ir.body_parts[0].models[0];
    assert!(m.vertices.is_empty() && m.meshes.is_empty());
    assert!(ir.notes.iter().any(|n| n.starts_with("meshes of body_ref")));
    assert_eq!(ir.bones.len(), 1);
}

#[test]
fn vtx107_strips_become_lod_triangles() {
    let mut ir = model(0, 44, &vec![skinned([255, 0, 0, 1], 1.0); 3]);
    let vtx = parse_vtx(&vtx107(99, 0)).unwrap();
    load_triangles(&mut ir, &vtx).unwrap();
    let lods = &ir.body_parts[0].models[0].lods;
    assert_eq!(lods[0].meshes[0].triangles, vec![[0, 2, 1]]);
    let stale = parse_vtx(&vtx107(98, 0)).unwrap();
    assert!(load_triangles(&mut ir, &stale).is_err());
    let static_prop = parse_vtx(&vtx107(99, 0x10)).unwrap();
    assert!(load_triangles(&mut ir, &static_prop).is_ok());
}
