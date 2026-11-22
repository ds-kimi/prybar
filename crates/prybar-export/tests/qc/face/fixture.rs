use prybar_mdl::ir::{Flex, FlexOp, FlexRule};
use prybar_mdl::ModelIr;

/// HL2 face layout: lid desc 3 with its lowerer/neutral/raiser descs 4-6,
/// a `<name>R+<name>L` pair and a rule-only desc 7.
pub fn hl2_face() -> ModelIr {
    let mut ir = crate::fixture::ir();
    ir.flex_descs.extend(
        ["upper_right", "upper_right_lowerer", "upper_right_neutral"]
            .into_iter()
            .chain(["upper_right_raiser", "right_open", "AU1R", "AU1L"])
            .map(String::from),
    );
    lids(&mut ir);
    ir.flex_rules = vec![
        FlexRule {
            flex: 7,
            ops: vec![FlexOp::Fetch(0)],
        },
        FlexRule {
            flex: 6,
            ops: vec![FlexOp::FetchFlex(7)],
        },
    ];
    ir
}

/// Lowerer and raiser keys tagged -11 / 11 the way studiomdl stores them.
fn lids(ir: &mut ModelIr) {
    let key = |desc, partner, targets| Flex {
        desc,
        partner,
        targets,
        ..Flex::default()
    };
    let face = &mut ir.body_parts[0].models[0];
    face.meshes[0].flexes = vec![
        key(3, 0, [-11.0, -10.0, -0.2, 0.1]),
        key(3, 0, [0.1, 0.2, 10.0, 11.0]),
        key(8, 9, [0.0, 1.0, 1.0, 1.0]),
    ];
    let eye = &mut face.eyeballs[0];
    eye.upper_flex_descs = [4, 5, 6];
    eye.upper_targets = [-0.2, 0.1, 0.3];
    eye.upper_lid_flex_desc = 3;
    eye.lower_lid_flex_desc = 3;
    eye.lower_flex_descs = [4, 5, 6];
}
