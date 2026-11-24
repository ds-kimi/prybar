mod compat;
mod face;
mod fixture;
mod fixture_anim;
mod fixture_flex;
mod gmod;
mod hostile;
mod ik;
mod parts;
mod vtmb;

use prybar_export::qc::{decompile_files, qc_files, Names, QcOptions};

#[test]
fn golden_single_qc() {
    let ir = fixture::ir();
    let files = qc_files(&ir, &Names::new(&ir), QcOptions::default());
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].name, "test.qc");
    insta::assert_snapshot!(files[0].text);
}

#[test]
fn split_layout_includes_every_section() {
    let ir = fixture::ir();
    let files = qc_files(&ir, &Names::new(&ir), QcOptions { split: true });
    let names: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
    let expected = [
        "test.qc",
        "test_model.qci",
        "test_bones.qci",
        "test_animations.qci",
        "test_collision.qci",
    ];
    assert_eq!(names, expected);
    insta::assert_snapshot!(files[0].text);
}

/// Every SMD/VTA/VRD the QC names must be among the written files.
#[test]
fn qc_references_exist() {
    let ir = fixture::ir();
    let files = decompile_files(&ir, QcOptions::default());
    let qc = &files.last().unwrap().text;
    let written: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
    for token in qc.split('"').skip(1).step_by(2) {
        let ext = token.rsplit('.').next().unwrap_or("");
        if ["smd", "vta", "vrd"].contains(&ext) {
            assert!(written.contains(&token), "{token} not written: {written:?}");
        }
    }
}
