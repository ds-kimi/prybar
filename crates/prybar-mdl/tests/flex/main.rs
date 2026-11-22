mod fixture;

use fixture::build;
use prybar_mdl::ir::FlexOp;

fn put(buf: &mut [u8], at: usize, v: i32) {
    buf[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

#[test]
fn parses_flex_records() {
    let ir = prybar_mdl::parse(&build()).unwrap();
    assert_eq!(ir.flex_descs, ["jaw_drop"]);
    let c = &ir.flex_controllers[0];
    assert_eq!(
        (c.kind.as_str(), c.name.as_str(), c.max),
        ("phoneme", "jaw_drop", 1.0)
    );
    let ops = &ir.flex_rules[0].ops;
    assert_eq!(ops, &[FlexOp::Fetch(0), FlexOp::Const(1.5), FlexOp::Mul]);
    let model = &ir.body_parts[0].models[0];
    let flex = &model.meshes[0].flexes[0];
    assert_eq!((flex.desc, flex.targets[1], flex.wrinkle), (0, 1.0, false));
    let d = &flex.deltas[0];
    assert_eq!((d.index, d.speed, d.position), (0, 255, [1.0, 0.0, 0.0]));
    assert_eq!(model.eyeballs[0].name, "phoneme");
    assert_eq!(model.eyeballs[0].radius, 0.5);
}

#[test]
fn rejects_forged_flex_counts() {
    for (at, v) in [
        (260, i32::MAX),
        (1728, 1 << 28),
        (860 + 16, -1),
        (1800, 1 << 30),
    ] {
        let mut b = build();
        put(&mut b, at, v);
        assert!(prybar_mdl::parse(&b).is_err(), "field {at} = {v}");
    }
}
