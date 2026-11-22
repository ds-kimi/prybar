//! Texture lookup over a synthetic game tree, plus an opt-in pass over a
//! real install (`PRYBAR_GAME_DIR=<.../garrysmod>`).

#[path = "../../prybar-vtf/tests/support/mod.rs"]
mod vtf_support;

use prybar_mdl::ir::{Material, ModelIr};
use prybar_service::texture::{base_texture, materials, texture, Sources};
use std::path::PathBuf;

#[test]
fn vmt_keys_and_patches() {
    let vmt =
        "\"VertexLitGeneric\"\n{\n  // comment\n  \"$baseTexture\" \"Models\\Crate\\Wood\"\n}";
    assert_eq!(base_texture(vmt), Some(Ok("Models\\Crate\\Wood".into())));
    let bare = "VertexLitGeneric { $basetexture models/a }";
    assert_eq!(base_texture(bare), Some(Ok("models/a".into())));
    let patch = "patch { include \"materials/models/a.vmt\" insert { $x 1 } }";
    assert_eq!(
        base_texture(patch),
        Some(Err("materials/models/a.vmt".into()))
    );
    assert_eq!(base_texture("\"unterminated"), None);
}

/// `root/models/props/crate.mdl`'s material `Crate` with a DXT1 texture.
fn tree(root: &std::path::Path) {
    std::fs::create_dir_all(root.join("models/props")).unwrap();
    std::fs::create_dir_all(root.join("materials/models/props")).unwrap();
    let vmt = "VertexLitGeneric { $basetexture \"models/props/wood\" }";
    std::fs::write(root.join("materials/models/props/crate.vmt"), vmt).unwrap();
    let vtf = vtf_support::vtf(5, 13, (4, 4), 1, &vtf_support::dxt1_red_blue());
    std::fs::write(root.join("materials/models/props/wood.vtf"), vtf).unwrap();
}

#[test]
fn finds_texture_next_to_the_model_tree() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    tree(root);
    let ir = ModelIr {
        materials: ["Crate", "gone"]
            .map(|n| Material {
                name: n.into(),
                flags: 0,
            })
            .to_vec(),
        material_dirs: vec!["models\\props\\".into()],
        ..Default::default()
    };
    let mdl = root.join("models/props/crate.mdl");
    let sources = Sources::for_model(&mdl.to_string_lossy(), &[]);
    let found = materials(&ir, &sources);
    assert_eq!(found[0].base_texture.as_deref(), Some("models/props/wood"));
    assert!(found[0].texture_location.is_some());
    assert_eq!(found[1].vmt, None);
    assert_eq!(
        found[1].tried,
        vec!["materials/models/props/gone.vmt".to_owned()]
    );
    let tex = texture(&sources, "models/props/wood", 0).unwrap();
    assert_eq!((tex.image.width, tex.format), (4, "DXT1"));
}

/// Decodes every model base texture of a real game; run by hand.
#[test]
#[ignore]
fn real_game_textures_decode() {
    let dir = PathBuf::from(std::env::var("PRYBAR_GAME_DIR").expect("PRYBAR_GAME_DIR"));
    let sources = Sources::for_model("", std::slice::from_ref(&dir));
    let mdl = std::env::var("PRYBAR_MDL")
        .unwrap_or_else(|_| format!("{}/garrysmod_dir.vpk:models/antlion.mdl", dir.display()));
    let ir = prybar_service::model::open(&mdl).unwrap();
    let mut decoded = 0;
    for m in materials(&ir, &sources) {
        let base = m
            .base_texture
            .unwrap_or_else(|| panic!("{} has no base texture: {:?}", m.name, m.tried));
        let tex = texture(&sources, &base, 0).unwrap();
        eprintln!(
            "{base}: {}x{} {} from {}",
            tex.image.width, tex.image.height, tex.format, tex.location
        );
        decoded += 1;
    }
    assert!(decoded > 0);
}
