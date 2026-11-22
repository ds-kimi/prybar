use super::fixture::hl2_face;
use prybar_export::qc::{qc_files, Names, QcOptions};
use prybar_mdl::ModelIr;

fn qc(ir: &ModelIr) -> String {
    qc_files(ir, &Names::new(ir), QcOptions::default())
        .remove(0)
        .text
}

/// Round-trip failures on eli/female_01: rules for `upper_right_raiser`
/// need an `eyelid` line, not `flex "upper_right"` frames.
#[test]
fn lid_frames_become_eyelid_lines() {
    let text = qc(&hl2_face());
    let line = text.lines().find(|l| l.contains("eyelid upper_right"));
    let line = line.unwrap_or_else(|| panic!("no eyelid line:\n{text}"));
    assert!(
        line.contains("lowerer 1 -0.2 neutral 0 0.1 raiser 2 0.3"),
        "{line}"
    );
    assert!(line.ends_with("eyeball \"eye_right\""), "{line}");
    assert!(!text.contains("flex \"upper_right\""), "{text}");
}

#[test]
fn rule_only_descs_are_localvars() {
    let text = qc(&hl2_face());
    assert!(text.contains("  localvar right_open\n"), "{text}");
    assert!(!text.contains("localvar upper_right_raiser"), "{text}");
}

/// studiomdl names the desc `<name>R` and the partner `<name>L`.
#[test]
fn right_first_pair_is_a_flexpair() {
    let text = qc(&hl2_face());
    assert!(text.contains("flexpair \"AU1\" 1 frame 3"), "{text}");
}
