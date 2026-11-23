mod big_endian;
mod fixture;
mod hostile;

const INCH: f32 = 1.0 / 0.0254;

#[test]
fn parses_synthetic_phy() {
    let phy = prybar_mdl::parse_phy(&fixture::build()).unwrap();
    assert_eq!(phy.checksum, fixture::CHECKSUM);
    assert_eq!(phy.solids.len(), 1);
    let piece = &phy.solids[0].pieces[0];
    assert_eq!(piece.bone, 0);
    // IVP (x, y, z) meters, Y up -> Source (x, z, -y) inches.
    assert_eq!(
        piece.triangles,
        vec![[[0.0; 3], [INCH, 0.0, 0.0], [0.0, INCH, 0.0]]]
    );
    assert_eq!(phy.text, fixture::TEXT);
    let solid = &phy.blocks[0];
    assert_eq!(
        (solid.name.as_str(), solid.get("name")),
        ("solid", Some("root"))
    );
    assert_eq!(solid.get("mass"), Some("5.0"));
}

#[test]
fn parses_legacy_solid_without_vphy() {
    let mut data = fixture::build();
    // Drop the 28-byte VPHY header and shrink the solid size to match.
    data.drain(20..48);
    let size = i32::from_le_bytes(data[16..20].try_into().unwrap()) - 28;
    data[16..20].copy_from_slice(&size.to_le_bytes());
    let phy = prybar_mdl::parse_phy(&data).unwrap();
    assert_eq!(phy.solids[0].pieces[0].triangles.len(), 1);
}

#[test]
fn forged_client_data_gives_no_bone() {
    let mut data = fixture::build();
    fixture::put(&mut data, fixture::LEDGE + 4, i32::MIN);
    let phy = prybar_mdl::parse_phy(&data).unwrap();
    assert_eq!(phy.solids[0].pieces[0].bone, -1);
}

#[test]
fn quoted_brace_is_a_value() {
    let mut data = fixture::build();
    data.truncate(data.len() - fixture::TEXT.len() - 1);
    data.extend_from_slice(b"solid { \"name\" \"}\" \"mass\" \"2\" }\0");
    let phy = prybar_mdl::parse_phy(&data).unwrap();
    let solid = &phy.blocks[0];
    assert_eq!(
        (solid.get("name"), solid.get("mass")),
        (Some("}"), Some("2"))
    );
}
