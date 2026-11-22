//! BMP export of GoldSrc embedded textures.

use prybar_export::goldsrc::{bmp, texture_files};
use prybar_mdl::ir::Texture;
use prybar_mdl::ModelIr;

#[path = "../../prybar-mdl/tests/goldsrc/shared.rs"]
mod gold;

fn tex(name: &str, w: u32, h: u32) -> Texture {
    Texture {
        name: name.into(),
        width: w,
        height: h,
        pixels: (0..w * h).map(|i| i as u8).collect(),
        palette: (0..768).map(|i| (i / 3) as u8).collect(),
        ..Texture::default()
    }
}

#[test]
fn bmp_layout_is_bottom_up_and_padded() {
    let b = bmp(&tex("a.bmp", 3, 2)).unwrap();
    assert_eq!(&b[0..2], b"BM");
    assert_eq!(
        u32::from_le_bytes(b[2..6].try_into().unwrap()) as usize,
        b.len()
    );
    assert_eq!(b.len(), 1078 + 4 * 2);
    assert_eq!(&b[54 + 4..54 + 8], &[1, 1, 1, 0]);
    // Last row first: pixels 3, 4, 5 then one pad byte.
    assert_eq!(&b[1078..1082], &[3, 4, 5, 0]);
    assert_eq!(&b[1082..1086], &[0, 1, 2, 0]);
}

#[test]
fn bad_sizes_are_skipped_and_names_are_unique() {
    let mut broken = tex("bad", 2, 2);
    broken.pixels.pop();
    let ir = ModelIr {
        textures: vec![tex("../Skin.BMP", 1, 1), tex("skin.bmp", 1, 1), broken],
        ..ModelIr::default()
    };
    let names: Vec<_> = texture_files(&ir).into_iter().map(|(n, _)| n).collect();
    assert_eq!(names, ["Skin.bmp", "skin_2.bmp"]);
}

#[test]
fn v10_model_exports_reference_anim_and_bmp() {
    let ir = prybar_mdl::parse(&gold::builder::build()).unwrap();
    let files = prybar_export::smd::mesh_files(&ir);
    let names: Vec<_> = files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names, ["box_ref.smd", "idle.smd"]);
    assert!(files[0].text.contains(
        "skin.bmp
  1 0.000000 2.000000 0.000000"
    ));
    let tex = texture_files(&ir);
    assert_eq!((tex[0].0.as_str(), tex[0].1.len()), ("skin.bmp", 1078 + 8));
}
