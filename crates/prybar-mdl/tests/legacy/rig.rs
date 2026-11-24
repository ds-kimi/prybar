//! 2531 pose parameters, procedural bones and bind pose rotation.

use super::builder::{build, put, put_str};
use super::records::BONE;
use prybar_mdl::ir::ProcBone;
use prybar_mdl::parse;

const RULE: usize = 1700;
const POSE: usize = 1800;

/// One bone with a 90 degree yaw, the rotation bit and a quaternion
/// rule with one trigger, plus one pose parameter.
fn rigged() -> Vec<u8> {
    let (mut buf, ints) = build(2531);
    let half = std::f32::consts::FRAC_1_SQRT_2.to_bits() as i32;
    put(&mut buf, BONE + 28, half);
    put(&mut buf, BONE + 32, half);
    put(&mut buf, BONE + 112, 0x02);
    put(&mut buf, BONE + 116, 2);
    put(&mut buf, BONE + 120, (RULE - BONE) as i32);
    put(&mut buf, RULE + 4, 1);
    put(&mut buf, RULE + 8, 12);
    put(&mut buf, RULE + 12, 2f32.to_bits() as i32);
    put(&mut buf, RULE + 56, 1f32.to_bits() as i32);
    put(&mut buf, ints + 39 * 4, 1);
    put(&mut buf, ints + 40 * 4, POSE as i32);
    put(&mut buf, POSE, 1960 - POSE as i32);
    put(&mut buf, POSE + 8, (-1f32).to_bits() as i32);
    put(&mut buf, POSE + 12, 1f32.to_bits() as i32);
    put_str(&mut buf, 1960, "move_x");
    buf
}

#[test]
fn v2531_rig_decodes() {
    let ir = parse(&rigged()).unwrap();
    let bone = &ir.bones[0];
    assert_eq!(bone.flags, 0x02);
    assert!((bone.rotation[2] - std::f32::consts::FRAC_PI_2).abs() < 1e-4);
    let Some(ProcBone::QuatInterp { control, triggers }) = &bone.procedural else {
        panic!("{:?}", bone.procedural);
    };
    assert_eq!((*control, triggers.len()), (0, 1));
    assert_eq!(triggers[0].tolerance, 0.5);
    assert_eq!(triggers[0].quat[3], 1.0);
    let p = &ir.pose_params[0];
    assert_eq!((p.name.as_str(), p.start, p.end), ("move_x", -1.0, 1.0));
    assert!(
        !ir.notes.iter().any(|n| n.contains("pose")),
        "{:?}",
        ir.notes
    );
}

#[test]
fn v2531_bad_rule_offset_errors_without_panic() {
    let mut buf = rigged();
    put(&mut buf, BONE + 120, 0x7fff_0000);
    assert!(parse(&buf).is_err());
}

/// One physics chain from the 2531 header slot after the surface prop.
#[test]
fn v2531_physics_chain_decodes() {
    let mut buf = rigged();
    let ints = build(2531).1;
    const CHAIN: usize = 2000;
    put(&mut buf, ints + 42 * 4, 1);
    put(&mut buf, ints + 43 * 4, CHAIN as i32);
    put(&mut buf, CHAIN + 4, -1);
    put(&mut buf, CHAIN + 8, 30f32.to_bits() as i32);
    put(&mut buf, CHAIN + 24, 45f32.to_bits() as i32);
    let chain = &parse(&buf).unwrap().physics_chains[0];
    assert_eq!((chain.bone, chain.parent), (0, -1));
    assert_eq!(chain.params, [30.0, 0.0, 0.0, 0.0, 45.0]);
}
