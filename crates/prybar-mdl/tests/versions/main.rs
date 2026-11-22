//! MDL v52 and v53 header-level parsing on synthetic files.

mod builder;
#[path = "../common/mod.rs"]
mod common;
mod hostile;

use prybar_mdl::{detect, embedded, inspect, parse, Family};

#[test]
fn v52_parses_like_v49() {
    let mut buf = common::build();
    buf[4..8].copy_from_slice(&52i32.to_le_bytes());
    assert_eq!(detect(&buf).unwrap().family, Family::Source52);
    let ir = parse(&buf).unwrap();
    let base = parse(&common::build()).unwrap();
    assert_eq!(ir.version, 52);
    assert_eq!((ir.name, ir.bones), (base.name, base.bones));
    assert_eq!(ir.body_parts, base.body_parts);
    assert_eq!(ir.hitbox_sets, base.hitbox_sets);
}

#[test]
fn v53_header_bones_and_anims() {
    let ir = parse(&builder::build()).unwrap();
    assert_eq!((ir.version, ir.checksum), (53, 77));
    assert_eq!(ir.name, "models/long/path/model.mdl");
    let bone = &ir.bones[0];
    assert_eq!((bone.name.as_str(), bone.parent), ("root", -1));
    assert_eq!(bone.position[0], 2.0);
    assert_eq!(bone.quat[3], 1.0);
    assert_eq!(bone.rotation_scale[0], 5.0);
    assert_eq!((bone.flags, bone.surface_prop.as_str()), (1, "metal"));
    assert_eq!(bone.contents, 3);
    let anim = &ir.animations[0];
    assert_eq!((anim.name.as_str(), anim.frame_count), ("idle", 12));
    assert_eq!((anim.fps, anim.flags), (30.0, 1));
    assert!(anim.frames.is_empty());
    assert_eq!(inspect(&builder::build()).unwrap().animations, 1);
}

#[test]
fn v53_embedded_files() {
    let buf = builder::build();
    let e = embedded(&buf).unwrap();
    assert_eq!(e.vtx.map(<[u8]>::len), Some(16));
    assert_eq!(e.vvd.map(<[u8]>::len), Some(8));
    assert!(e.phy.is_none());
    assert!(embedded(&common::build()).is_err());
    let mut bad = buf.clone();
    builder::put(&mut bad, &[(444, i32::MAX)]);
    assert!(embedded(&bad).is_err());
}
