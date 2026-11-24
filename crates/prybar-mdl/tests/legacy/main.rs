//! MDL 2531 and v27-v37 parsing on synthetic files.

mod builder;
mod face;
mod geometry;
mod hostile;
mod inline;
mod records;
mod vtx107;

use prybar_mdl::{detect, inspect, parse, Family};

fn check(version: i32) -> prybar_mdl::ModelIr {
    let (buf, _) = builder::build(version);
    assert_eq!(detect(&buf).unwrap().family, Family::SourceLegacy);
    let ir = parse(&buf).unwrap();
    assert_eq!((ir.version, ir.checksum), (version, 99));
    assert_eq!(ir.name, "legacy/box.mdl");
    assert_eq!(ir.eye_position[0], 7.0);
    assert_eq!((ir.bones.len(), ir.bones[0].name.as_str()), (1, "root"));
    assert_eq!(ir.bones[0].parent, -1);
    let set = &ir.hitbox_sets[0];
    assert_eq!((set.name.as_str(), set.hitboxes[0].group), ("default", 2));
    assert_eq!(ir.materials[0].name, "skin0");
    let part = &ir.body_parts[0];
    assert_eq!(
        (part.name.as_str(), part.models[0].name.as_str()),
        ("body", "body_ref")
    );
    assert_eq!(part.models[0].bounding_radius, 9.0);
    assert!(ir.notes.iter().any(|n| n.starts_with("animations: 3")));
    assert_eq!(
        ir.notes.iter().any(|n| n.starts_with("meshes")),
        version != 2531
    );
    assert_eq!(inspect(&buf).unwrap().notes, ir.notes);
    ir
}

#[test]
fn v2531_parses() {
    assert_eq!(check(2531).attachments[0].name, "muzzle");
}

#[test]
fn v31_parses_without_attachments() {
    let ir = check(31);
    assert!(ir.attachments.is_empty());
    assert!(ir.notes.iter().any(|n| n.starts_with("attachments")));
    let json = serde_json::to_string(&ir).unwrap();
    assert!(json.contains("\"notes\""));
}

#[test]
fn v32_v36_v37_parse() {
    for v in [32, 36, 37] {
        assert_eq!(check(v).attachments[0].name, "muzzle");
    }
}
