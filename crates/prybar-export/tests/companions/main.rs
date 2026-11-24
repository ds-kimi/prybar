mod fixture;

use prybar_mdl::ir::{ConvexPiece, KvBlock, PhysSolid, Physics};

#[test]
fn vta_merges_shared_flexes_into_one_frame() {
    let files = prybar_export::vta::vta_files(&fixture::ir());
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].name, "face.vta");
    let text = &files[0].text;
    let anim = text.split_once("vertexanimation\n").unwrap().1;
    // Frame 1 moves vertex 1 of both meshes; frame 2 only mesh 1.
    let expected = "time 1 # smile\n  1 1.500000 0.000000 0.000000 0.000000 0.000000 1.000000\n  3 3.250000 0.000000 0.000000 0.000000 0.000000 1.000000\ntime 2 # blink\n  3 5.000000 0.000000 0.000000 0.000000 0.000000 1.000000\nend\n";
    assert!(anim.ends_with(expected), "{anim}");
    assert!(
        text.contains("skeleton\ntime 0 # basis shape key\ntime 1 # smile\ntime 2 # blink\nend\n")
    );
}

#[test]
fn vrd_writes_quaternion_helpers() {
    let text = prybar_export::vrd::vrd(&fixture::ir()).unwrap();
    let expected = "\n<helper> helper root root arm\n<basepos> 0 0 0\n<trigger> 90.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 1.000000 2.000000 3.000000\n";
    assert_eq!(text, expected);
    let mut ir = fixture::ir();
    ir.bones[2].procedural = None;
    assert!(prybar_export::vrd::vrd(&ir).is_none());
}

#[test]
fn physics_smd_moves_ragdoll_solids_to_their_bone() {
    let mut ir = fixture::ir();
    let solid = |bone| PhysSolid {
        pieces: vec![ConvexPiece {
            bone,
            triangles: vec![[[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]],
        }],
    };
    let block = |i: &str, name: &str| KvBlock {
        name: "solid".into(),
        pairs: vec![("index".into(), i.into()), ("name".into(), name.into())],
    };
    ir.physics = Some(Physics {
        solids: vec![solid(-1), solid(-1)],
        blocks: vec![block("0", "ValveBiped.root"), block("1", "valvebiped.ARM")],
        ..Physics::default()
    });
    let smd = prybar_export::smd::physics_smd(&ir).unwrap();
    let tris = smd.split_once("triangles\n").unwrap().1;
    assert!(tris.contains("  0 1.000000 0.000000 0.000000 0.000000 0.000000 1.000000"));
    // Solid 1 sits in bone 1's space, one unit along X in the bind pose.
    assert!(tris.contains(
        "  1 2.000000 0.000000 0.000000 0.000000 0.000000 1.000000 0.000000 0.000000 1 1 1.000000"
    ));
    assert!(prybar_export::smd::physics_smd(&fixture::ir()).is_none());
}

/// VTMB solids carry their own `origin`/`angles` pose, which wins over the
/// MDL bind; a pose missing one key is ignored.
#[test]
fn physics_smd_uses_solid_reference_pose() {
    let mut ir = fixture::ir();
    ir.version = 2531;
    let tri = [[1.0, 0.0, 0.0], [0.0; 3], [0.0, 0.0, 1.0]];
    let pieces = vec![ConvexPiece {
        bone: -1,
        triangles: vec![tri],
    }];
    let solid = PhysSolid { pieces };
    let block = |i: &str, extra: &[(&str, &str)]| KvBlock {
        name: "solid".into(),
        pairs: [("index", i), ("name", "valvebiped.arm")]
            .iter()
            .chain(extra)
            .map(|(k, v)| ((*k).into(), (*v).into()))
            .collect(),
    };
    ir.physics = Some(Physics {
        solids: vec![solid.clone(), solid],
        blocks: vec![
            block("0", &[("origin", "0 0 10"), ("angles", "0 90 0")]),
            block("1", &[("origin", "0 0 10")]),
        ],
        ..Physics::default()
    });
    let smd = prybar_export::smd::physics_smd(&ir).unwrap();
    assert!(smd.contains("  1 0.000000 1.000000 10.000000 "), "{smd}");
    // The partial pose falls back to the bind: one unit along X.
    assert!(smd.contains("  1 2.000000 0.000000 0.000000 "), "{smd}");
}
