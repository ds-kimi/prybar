mod common;

use prybar_mdl::{detect, inspect, parse, Error, Family};

#[test]
fn parses_synthetic_v49() {
    let ir = parse(&common::build()).unwrap();
    assert_eq!(ir.version, 49);
    assert_eq!(ir.name, "test/box.mdl");
    assert_eq!(ir.bones[0].name, "root");
    assert_eq!(ir.bones[0].parent, -1);
    assert_eq!(ir.bones[0].surface_prop, "metal");
    assert_eq!(ir.materials[0].name, "box");
    assert_eq!(ir.material_dirs, ["models/test/"]);
    assert_eq!(ir.skins, vec![vec![0], vec![1]]);
    assert_eq!(ir.body_parts[0].name, "body");
    assert_eq!(ir.body_parts[0].models[0].name, "box");
    assert_eq!(ir.body_parts[0].models[0].meshes[0].vertex_count, 8);
    assert_eq!(ir.hitbox_sets[0].hitboxes[0].name, "head");
    assert_eq!(ir.attachments[0].name, "tip");
    assert_eq!(ir.surface_prop, "flesh");
    assert_eq!(ir.key_values, "mdlkeyvalue{}");
}

#[test]
fn inspect_summarizes() {
    let s = inspect(&common::build()).unwrap();
    assert_eq!((s.bones, s.models, s.meshes, s.hitboxes), (1, 1, 1, 1));
    assert_eq!(s.family, Family::Source44);
    serde_json::to_string(&s).unwrap();
}

#[test]
fn detects_families() {
    let mut buf = common::build();
    for (v, fam) in [
        (10, Family::GoldSrc),
        (37, Family::SourceLegacy),
        (53, Family::Source53),
    ] {
        buf[4..8].copy_from_slice(&i32::to_le_bytes(v));
        assert_eq!(detect(&buf).unwrap().family, fam);
    }
    assert!(matches!(detect(b"NOPE\0\0\0\0"), Err(Error::BadMagic(_))));
}

#[test]
fn hostile_input_never_panics() {
    let good = common::build();
    for at in (0..good.len()).step_by(4) {
        for v in [-1i32, i32::MAX, i32::MIN, 0x7fff] {
            let mut buf = good.clone();
            buf[at..at + 4].copy_from_slice(&v.to_le_bytes());
            let _ = parse(&buf);
        }
    }
    for len in 0..good.len() {
        let _ = parse(&good[..len]);
    }
}
