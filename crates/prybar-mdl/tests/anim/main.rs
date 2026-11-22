mod fixture;
mod hostile;

fn xs(ir: &prybar_mdl::ModelIr) -> Vec<f32> {
    ir.animations[0]
        .frames
        .iter()
        .map(|f| f[0].position[0])
        .collect()
}

#[test]
fn decodes_rle_positions_and_raw_rotation() {
    let ir = prybar_mdl::parse(&fixture::build(false)).unwrap();
    let a = &ir.animations[0];
    assert_eq!((a.name.as_str(), a.fps, a.frame_count), ("@walk", 30.0, 3));
    assert_eq!(xs(&ir), [11.0, 21.0, 21.0]);
    let root = &a.frames[0][0];
    assert_eq!((root.position[1], root.position[2]), (2.0, 3.0));
    assert!((root.quat[3] - 1.0).abs() < 1e-6, "{:?}", root.quat);
}

#[test]
fn reads_sequence_and_event() {
    let ir = prybar_mdl::parse(&fixture::build(false)).unwrap();
    let s = &ir.sequences[0];
    assert_eq!(
        (s.name.as_str(), s.activity_name.as_str()),
        ("walk", "ACT_WALK")
    );
    assert_eq!((s.group_size, s.anims.as_slice()), ([1, 1], &[0i16][..]));
    let e = &s.events[0];
    assert_eq!(
        (e.cycle, e.event, e.options.as_str(), e.name.as_str()),
        (0.5, 7, "foo", "AE_FOO")
    );
}

#[test]
fn ani_block_needs_the_ani_file() {
    let mdl = fixture::build(true);
    let without = prybar_mdl::parse(&mdl).unwrap();
    assert!(without.animations[0].frames.is_empty());
    assert_eq!(without.animations[0].frame_count, 3);
    let with = prybar_mdl::parse_with_ani(&mdl, Some(&fixture::ani())).unwrap();
    assert_eq!(xs(&with), [11.0, 21.0, 21.0]);
}
