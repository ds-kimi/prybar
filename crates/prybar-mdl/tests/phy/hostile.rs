use super::fixture::{build, put, LEDGE};

#[test]
fn rejects_every_truncation() {
    let data = build();
    let text_start = data.len() - super::fixture::TEXT.len() - 1;
    for cut in 0..text_start {
        assert!(prybar_mdl::parse_phy(&data[..cut]).is_err(), "cut {cut}");
    }
}

#[test]
fn rejects_forged_counts_and_sizes() {
    for (at, v) in [
        (0, 4),
        (0, -1),
        (8, i32::MAX),
        (8, -3),
        (16, i32::MAX),
        (16, -8),
    ] {
        let mut data = build();
        put(&mut data, at, v);
        assert!(prybar_mdl::parse_phy(&data).is_err(), "field {at} = {v}");
    }
}

#[test]
fn rejects_forged_ledges() {
    // Triangle count past the solid, negative, then a point index far out.
    let cases = [
        (LEDGE + 12, 0x7fff),
        (LEDGE + 12, 0xffff),
        (LEDGE + 16 + 8, 900),
    ];
    for (at, v) in cases {
        let mut data = build();
        put(&mut data, at, v);
        assert!(prybar_mdl::parse_phy(&data).is_err(), "field {at} = {v}");
    }
}

#[test]
fn stops_on_point_offset_loops() {
    // A ledge whose points sit before it ends the walk instead of looping.
    let mut data = build();
    put(&mut data, LEDGE, -64);
    let phy = prybar_mdl::parse_phy(&data);
    assert!(phy.is_err() || phy.unwrap().solids[0].pieces.len() == 1);
}
