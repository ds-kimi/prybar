mod common;

use common::geometry::{vtx, vvd};
use prybar_mdl::{load_geometry, parse, parse_vtx, parse_vvd, Error};

#[test]
fn builds_triangles_from_vvd_and_vtx() {
    let mut ir = parse(&common::build()).unwrap();
    let vtx = parse_vtx(&vtx()).unwrap();
    load_geometry(&mut ir, &parse_vvd(&vvd()).unwrap(), &vtx).unwrap();
    let model = &ir.body_parts[0].models[0];
    assert_eq!(model.vertices.len(), 8);
    assert_eq!(model.vertices[3].position[0], 3.0);
    assert_eq!(model.vertices[0].weights, vec![(0, 1.0)]);
    let tris = &model.lods[0].meshes[0].triangles;
    assert_eq!(tris.len(), vtx.body_parts[0][0][0].meshes[0].len());
    assert_eq!(tris, &[[0, 2, 1], [2, 3, 1]]);
}

#[test]
fn rejects_mismatched_checksum() {
    let mut ir = parse(&common::build()).unwrap();
    ir.checksum = 1;
    let res = load_geometry(
        &mut ir,
        &parse_vvd(&vvd()).unwrap(),
        &parse_vtx(&vtx()).unwrap(),
    );
    assert!(matches!(res, Err(Error::Invalid { .. })));
}

#[test]
fn hostile_companions_never_panic() {
    for good in [vvd(), vtx()] {
        for at in (0..good.len()).step_by(2) {
            for v in [0xffu8, 0x7f, 0x80] {
                let mut buf = good.clone();
                buf[at] = v;
                let _ = parse_vvd(&buf).map(|_| ());
                let _ = parse_vtx(&buf).map(|_| ());
            }
        }
    }
}
