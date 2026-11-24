//! MDL 2531 flexes, eyeballs and phoneme filter.

use super::builder::{put, put_str};
use super::inline::{buffer, skinned, MESH, MODEL};
use prybar_mdl::parse;

const DESC: usize = 3000;
const FLEX: usize = 3040;
const DELTA: usize = 3080;
const EYE: usize = 3100;

/// One flex desc, one packed flex on the mesh and one eyeball bound to
/// the mesh through its material type and parameter.
fn face_buffer() -> Vec<u8> {
    let (mut buf, ints) = buffer(0, 44, &[skinned([255, 0, 0, 1], 1.0)]);
    let float = |v: f32| v.to_bits() as i32;
    for (slot, v) in [(1, float(0.1)), (2, float(0.3)), (29, 1), (30, DESC as i32)] {
        put(&mut buf, ints + slot * 4, v);
    }
    put(&mut buf, DESC, 8);
    put_str(&mut buf, DESC + 8, "jaw");
    // Mesh words 4-7: flex count and index, material type and parameter.
    let flexes = (FLEX - MESH) as i32;
    for (at, v) in [(16, 1), (20, flexes), (24, 1), (28, 0)] {
        put(&mut buf, MESH + at, v);
    }
    flex(&mut buf);
    eyeball(&mut buf);
    buf
}

fn flex(buf: &mut [u8]) {
    put(buf, FLEX + 8, 1f32.to_bits() as i32);
    buf[FLEX + 20..FLEX + 22].copy_from_slice(&1i16.to_le_bytes());
    put(buf, FLEX + 24, (DELTA - FLEX) as i32);
    buf[FLEX + 28..FLEX + 30].copy_from_slice(&1i16.to_le_bytes());
    let top = (5313u16 * 12).to_le_bytes();
    buf[DELTA..DELTA + 8].copy_from_slice(&[0, 0, top[0], top[1], top[0], top[1], 255, 255]);
}

fn eyeball(buf: &mut [u8]) {
    let f = MODEL + 128;
    put(buf, f + 16 * 4, 1);
    put(buf, f + 17 * 4, (EYE - MODEL) as i32);
    put(buf, EYE, 140);
    put_str(buf, EYE + 140, "eye_r");
    put(buf, EYE + 6 * 4, 0.5f32.to_bits() as i32);
    put(buf, EYE + 13 * 4, -5);
}

#[test]
fn v2531_flexes_eyeballs_and_phoneme_filter_decode() {
    let ir = parse(&face_buffer()).unwrap();
    assert_eq!(ir.flex_descs, vec!["jaw".to_string()]);
    assert_eq!(ir.phoneme_filter, [0.1, 0.3]);
    let m = &ir.body_parts[0].models[0];
    let flex = &m.meshes[0].flexes[0];
    assert_eq!((m.meshes[0].flex_count, flex.targets[1]), (1, 1.0));
    let d = &flex.deltas[0];
    assert_eq!((d.position, d.normal), ([0.0, 0.0, 8.0], [0.0, 0.0, 2.0]));
    let eye = &m.eyeballs[0];
    assert_eq!(
        (eye.name.as_str(), eye.radius, eye.texture),
        ("eye_r", 0.5, 0)
    );
    assert!(
        !ir.notes.iter().any(|n| n.starts_with("flex")),
        "{:?}",
        ir.notes
    );
}
