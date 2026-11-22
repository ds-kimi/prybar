use super::parts::{bones, face, sub};
use prybar_mdl::ir::*;
use prybar_mdl::ModelIr;

fn markers(ir: &mut ModelIr) {
    let local = [
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 2.0],
        [0.0, 0.0, 1.0, 3.0],
    ];
    ir.attachments = vec![Attachment {
        name: "eyes".into(),
        bone: 1,
        flags: 0x10000,
        local,
    }];
    let hitbox = Hitbox {
        name: String::new(),
        bone: 1,
        group: 1,
        min: [-1.0; 3],
        max: [1.0; 3],
    };
    ir.hitbox_sets = vec![HitboxSet {
        name: "default".into(),
        hitboxes: vec![hitbox],
    }];
}

fn header(ir: &mut ModelIr) {
    ir.name = "prybar/test.mdl".into();
    ir.flags = 0x10;
    ir.surface_prop = "metal".into();
    ir.eye_position = [0.0, 0.0, 64.0];
    (ir.hull_min, ir.hull_max) = ([-1.0; 3], [1.0; 3]);
    ir.material_dirs = vec!["models/prybar/".into()];
    let material = |n: &str| Material {
        name: n.into(),
        flags: 0,
    };
    ir.materials = ["skin_a", "eyeball_r", "skin_b"].map(material).to_vec();
    ir.skins = vec![vec![0, 1], vec![2, 1]];
    ir.include_models = vec![IncludeModel {
        label: String::new(),
        path: "prybar/anims.mdl".into(),
    }];
    ir.key_values = "prop_data\n{\n\"base\" \"Metal.Small\"\n}".into();
}

/// Synthetic IR touching every QC section.
pub fn ir() -> ModelIr {
    let mut ir = super::fixture_anim::anims();
    header(&mut ir);
    markers(&mut ir);
    ir.bones = bones();
    let part = |name: &str, models| BodyPart {
        name: name.into(),
        base: 1,
        models,
    };
    let hats = vec![sub("hat", 2, vec![]), SubModel::default()];
    ir.body_parts = vec![part("head", vec![face()]), part("hats", hats)];
    ir.flex_descs = vec!["smileL".into(), "smileR".into(), "blink".into()];
    ir.flex_controllers = super::fixture_flex::controllers();
    ir.flex_rules = super::fixture_flex::rules();
    ir.physics = Some(super::fixture_flex::physics());
    ir
}
