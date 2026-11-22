//! GoldSrc v10 parsing on a synthetic model built by our own writer.

mod builder;
mod companions;
mod hostile;
mod mesh_records;
mod records;

use prybar_mdl::{detect, inspect, parse, Family};

fn close(a: [f32; 3], b: [f32; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5)
}

#[test]
fn v10_header_skeleton_and_markers() {
    let buf = builder::build();
    assert_eq!(detect(&buf).unwrap().family, Family::GoldSrc);
    let ir = parse(&buf).unwrap();
    assert_eq!((ir.version, ir.name.as_str()), (10, "gold/box.mdl"));
    assert_eq!(ir.eye_position[0], 5.0);
    let names: Vec<_> = ir
        .bones
        .iter()
        .map(|b| (b.name.as_str(), b.parent))
        .collect();
    assert_eq!(names, [("root", -1), ("child", 0)]);
    assert_eq!(ir.attachments[0].name, "muzzle");
    assert_eq!(
        (ir.attachments[0].bone, ir.attachments[0].local[0][3]),
        (1, 4.0)
    );
    let hb = &ir.hitbox_sets[0].hitboxes[0];
    assert_eq!((hb.bone, hb.group, hb.max[0]), (1, 3, 2.0));
    assert_eq!(ir.materials[0].name, "skin.bmp");
    assert_eq!(ir.skins, [vec![0u16]]);
    let tex = &ir.textures[0];
    assert_eq!(
        (tex.width, tex.height, tex.pixels.as_slice()),
        (2, 2, &[0u8, 1, 2, 3][..])
    );
    assert_eq!(tex.palette.len(), 768);
    assert!(ir.notes.is_empty(), "{:?}", ir.notes);
}

#[test]
fn v10_mesh_is_in_model_space() {
    let ir = parse(&builder::build()).unwrap();
    let model = &ir.body_parts[0].models[0];
    assert_eq!(
        (ir.body_parts[0].name.as_str(), model.name.as_str()),
        ("body", "box_ref")
    );
    let tris = &model.lods[0].meshes[0].triangles;
    assert_eq!(tris.len(), 1);
    let v: Vec<_> = tris[0]
        .iter()
        .map(|&i| &model.vertices[i as usize])
        .collect();
    // Strip winding: corners 0, 2, 1.
    assert!(close(v[0].position, [0.0, 2.0, 0.0]), "{:?}", v[0].position);
    assert!(close(v[1].position, [0.0, 1.0, 1.0]), "{:?}", v[1].position);
    assert!(
        close(v[2].position, [-1.0, 1.0, 0.0]),
        "{:?}",
        v[2].position
    );
    assert!(close(v[0].normal, [0.0, 0.0, 1.0]));
    assert_eq!((v[1].uv, v[2].uv), ([0.0, 1.0], [1.0, 0.0]));
    assert_eq!(v[0].weights, [(1, 1.0)]);
}

#[test]
fn v10_sequence_and_frames() {
    let buf = builder::build();
    let ir = parse(&buf).unwrap();
    let seq = &ir.sequences[0];
    assert_eq!(
        (seq.name.as_str(), seq.flags, seq.anims.as_slice()),
        ("idle", 1, &[0i16][..])
    );
    assert_eq!(
        (seq.events[0].event, seq.events[0].options.as_str()),
        (5001, "10")
    );
    assert_eq!(seq.events[0].cycle, 1.0);
    let anim = &ir.animations[0];
    assert_eq!(
        (anim.fps, anim.frame_count, anim.frames.len()),
        (30.0, 2, 2)
    );
    assert!(close(anim.frames[0][1].position, [2.0, 0.0, 0.0]));
    assert!(close(anim.frames[1][1].position, [3.0, 0.0, 0.0]));
    let s = inspect(&buf).unwrap();
    assert_eq!((s.sequences, s.animations, s.meshes), (1, 1, 1));
}
