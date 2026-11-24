use prybar_export::qc::{qc_files, Names, QcOptions};
use prybar_mdl::ir::IncludeModel;

/// Crowbar issue #16: escape-aware studiomdl builds misread `\`.
#[test]
fn game_paths_use_forward_slashes() {
    let mut ir = super::fixture::ir();
    ir.name = r"props\crate.mdl".into();
    ir.material_dirs = vec![r"models\props\".into()];
    ir.include_models = vec![IncludeModel {
        path: r"models\anims\shared.mdl".into(),
        ..IncludeModel::default()
    }];
    let text = qc_files(&ir, &Names::new(&ir), QcOptions::default())
        .remove(0)
        .text;
    assert!(text.contains("$modelname \"props/crate.mdl\""));
    assert!(text.contains("$cdmaterials \"models/props/\""));
    assert!(text.contains("$includemodel \"models/anims/shared.mdl\""));
    assert!(!text.contains('\\'), "{text}");
}

/// Crowbar PR #18: ragdolls with concave bones need `$concaveperjoint`.
#[test]
fn ragdoll_with_concave_bone_gets_per_joint_flag() {
    let text = {
        let ir = super::fixture::ir();
        qc_files(&ir, &Names::new(&ir), QcOptions::default())
            .remove(0)
            .text
    };
    assert!(text.contains("$concaveperjoint"));
}

#[test]
fn bloodlines_phoneme_filter_round_trips_to_qc() {
    let mut ir = super::fixture::ir();
    let qc = |ir: &prybar_mdl::ModelIr| qc_files(ir, &Names::new(ir), QcOptions::default());
    assert!(!qc(&ir)[0].text.contains("$phonemefilter"));
    ir.phoneme_filter = [0.1, 0.3];
    assert!(qc(&ir)[0].text.contains("$phonemefilter 0.1 0.3\n"));
}
