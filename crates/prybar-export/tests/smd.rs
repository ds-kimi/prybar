#[path = "../../prybar-mdl/tests/common/mod.rs"]
mod common;

use prybar_export::smd::mesh_files;
use prybar_mdl::{load_geometry, parse, parse_vtx, parse_vvd};

#[test]
fn writes_reference_smd() {
    let mut ir = parse(&common::build()).unwrap();
    let vvd = parse_vvd(&common::geometry::vvd()).unwrap();
    let vtx = parse_vtx(&common::geometry::vtx()).unwrap();
    load_geometry(&mut ir, &vvd, &vtx).unwrap();
    let files = mesh_files(&ir);
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].name, "box.smd");
    let text = &files[0].text;
    assert!(text.starts_with("version 1\nnodes\n  0 \"root\" -1\nend\n"));
    assert_eq!(text.matches("\nbox\n").count(), 2);
    assert!(text.contains("  0 2.000000 0.000000 0.000000"));
    assert!(text.ends_with("end\n"));
}

#[test]
fn duplicate_model_names_get_unique_files() {
    let mut ir = parse(&common::build()).unwrap();
    let vvd = parse_vvd(&common::geometry::vvd()).unwrap();
    let vtx = parse_vtx(&common::geometry::vtx()).unwrap();
    load_geometry(&mut ir, &vvd, &vtx).unwrap();
    let copy = ir.body_parts[0].clone();
    ir.body_parts.push(copy);
    let names: Vec<_> = mesh_files(&ir).into_iter().map(|f| f.name).collect();
    assert_eq!(names, ["box.smd", "box_2.smd"]);
}
