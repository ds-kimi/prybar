use prybar_export::qc::{qc_files, Names, QcOptions};
use prybar_mdl::ir::PhysicsChain;

/// VTMB `$physicschain` is commented out (stock studiomdl rejects it),
/// written without a `.phy`, and skipped when its bone has no name.
#[test]
fn physics_chains_are_qc_comments() {
    let mut ir = super::fixture::ir();
    ir.physics = None;
    let chain = |bone, parent| PhysicsChain {
        bone,
        parent,
        params: [30.0, -1.5, 0.25, 2.0, 45.0],
    };
    ir.physics_chains = vec![chain(1, 0), chain(99, 0), chain(1, -1)];
    let text = &qc_files(&ir, &Names::new(&ir), QcOptions::default())[0].text;
    let (bone, parent) = (&ir.bones[1].name, &ir.bones[0].name);
    let with_parent = format!("// $physicschain \"{bone}\" \"{parent}\" 30 -1.5 0.25 2 45\n");
    assert!(text.contains(&with_parent), "{text}");
    assert!(text.contains(&format!("// $physicschain \"{bone}\" \"\" 30")));
    assert_eq!(text.matches("$physicschain").count(), 2);
}
