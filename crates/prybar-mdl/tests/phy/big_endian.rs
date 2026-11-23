//! The synthetic `.phy` of `fixture.rs`, written the way console ports
//! store it: every scalar big-endian, strings unchanged.

const INCH: f32 = 1.0 / 0.0254;

fn be(out: &mut Vec<u8>, v: &[i32]) {
    for x in v {
        out.extend_from_slice(&x.to_be_bytes());
    }
}

/// Ledge header (triangle count is an i16 + pad), one triangle whose edge
/// point indices are u16 + pad, then three points in meters.
fn ledge(out: &mut Vec<u8>) {
    be(out, &[32, 1, 0]);
    out.extend_from_slice(&[0, 1, 0, 0]);
    out.extend_from_slice(&[0; 4]);
    for index in [0u16, 1, 2] {
        out.extend_from_slice(&index.to_be_bytes());
        out.extend_from_slice(&[0, 0]);
    }
    let pts: [f32; 12] = [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    for p in pts {
        out.extend_from_slice(&p.to_be_bytes());
    }
}

fn build() -> Vec<u8> {
    let mut out = Vec::new();
    be(&mut out, &[16, 0, 1, 7]);
    let mut solid = b"YHPV".to_vec();
    solid.extend_from_slice(&[1, 0, 0, 0]);
    solid.extend_from_slice(&[0; 64]);
    solid.extend_from_slice(b"SPVI");
    ledge(&mut solid);
    be(&mut out, &[solid.len() as i32]);
    out.extend_from_slice(&solid);
    out.extend_from_slice(b"solid {\n\"index\" \"0\"\n}\n\0");
    out
}

#[test]
fn parses_big_endian_phy() {
    let phy = prybar_mdl::parse_phy(&build()).unwrap();
    assert_eq!(phy.checksum, 7);
    let piece = &phy.solids[0].pieces[0];
    assert_eq!(piece.bone, 0);
    assert_eq!(
        piece.triangles,
        vec![[[0.0; 3], [INCH, 0.0, 0.0], [0.0, INCH, 0.0]]]
    );
    assert_eq!(phy.blocks[0].get("index"), Some("0"));
}
