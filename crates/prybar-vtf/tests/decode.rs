mod support;

use prybar_vtf::Vtf;

#[test]
fn dxt1_block_decodes_both_colors() {
    let file = support::vtf(5, 13, (4, 4), 1, &support::dxt1_red_blue());
    let img = Vtf::parse(file).unwrap().decode(0).unwrap();
    assert_eq!((img.width, img.height), (4, 4));
    assert_eq!(&img.rgba[..4], &[255, 0, 0, 255]);
    assert_eq!(&img.rgba[12..16], &[0, 0, 255, 255]);
}

#[test]
fn plain_formats_swizzle_to_rgba() {
    let cases: [(i32, &[u8], [u8; 4]); 5] = [
        (12, &[1, 2, 3, 4], [3, 2, 1, 4]),
        (3, &[1, 2, 3], [3, 2, 1, 255]),
        (0, &[1, 2, 3, 4], [1, 2, 3, 4]),
        (5, &[9], [9, 9, 9, 255]),
        (6, &[9, 7], [9, 9, 9, 7]),
    ];
    for (id, px, want) in cases {
        let img = Vtf::parse(support::vtf(2, id, (1, 1), 1, px))
            .unwrap()
            .decode(0)
            .unwrap();
        assert_eq!(img.rgba, want, "format {id}");
    }
}

#[test]
fn mips_are_found_smallest_first() {
    // 2x2 BGR888 with mips: 1x1 (blue-ish) stored before 2x2 (white).
    let mut data = vec![10, 20, 30];
    data.extend([255u8; 12]);
    let vtf = Vtf::parse(support::vtf(1, 3, (2, 2), 2, &data)).unwrap();
    assert_eq!(vtf.decode(1).unwrap().rgba, [30, 20, 10, 255]);
    assert_eq!(vtf.decode(0).unwrap().rgba, [255; 16]);
    assert!(vtf.decode(2).is_err());
}

#[test]
fn dxt5_alpha_endpoints() {
    let mut block = [0u8; 16];
    block[0] = 200; // a0
    block[1] = 10; // a1, every index 0 -> a0
    block[8..16].copy_from_slice(&support::dxt1_red_blue());
    let img = Vtf::parse(support::vtf(4, 15, (4, 4), 1, &block))
        .unwrap()
        .decode(0)
        .unwrap();
    assert!(img.rgba.chunks(4).all(|p| p[3] == 200));
    assert_eq!(
        Vtf::parse(support::vtf(4, 15, (4, 4), 1, &block))
            .unwrap()
            .format_name(),
        "DXT5"
    );
}
