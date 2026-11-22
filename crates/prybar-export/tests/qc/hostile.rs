use proptest::prelude::*;
use prybar_export::qc::{decompile_files, QcOptions};
use prybar_mdl::ir::*;
use prybar_mdl::ModelIr;

/// Indices from a crafted MDL point anywhere; the writer must still
/// produce text and never panic.
fn hostile_ir(index: i32, count: i32, name: String) -> ModelIr {
    let mut ir = super::fixture::ir();
    ir.name = name.clone();
    ir.bones[1].parent = index;
    ir.attachments[0].bone = index;
    ir.sequences[0].anims = vec![index as i16, i16::MIN];
    ir.sequences[0].param_index = [index, count];
    ir.sequences[0].group_size = [count, index];
    ir.sequences[0].auto_layers[0].sequence = index as i16;
    ir.body_parts[0].models[0].eyeballs[0].bone = index;
    ir.body_parts[0].models[0].eyeballs[0].texture = count;
    ir.flex_rules.push(FlexRule {
        flex: index,
        ops: vec![FlexOp::NWay(index), FlexOp::Dominate(count)],
    });
    ir.flex_controllers[0].name = name;
    ir.skins.push(vec![u16::MAX]);
    ir
}

proptest! {
    #[test]
    fn never_panics(index in any::<i32>(), count in any::<i32>(), name in ".{0,40}") {
        let ir = hostile_ir(index, count, name);
        let files = decompile_files(&ir, QcOptions { split: true });
        prop_assert!(files.iter().any(|f| f.name.ends_with(".qc")));
    }
}

/// A newline or quote in a name must not start a new QC command.
#[test]
fn names_cannot_inject_commands() {
    let ir = hostile_ir(0, 0, "x\"\n$include \"evil.qci".into());
    let files = decompile_files(&ir, QcOptions::default());
    let qc = &files.last().unwrap().text;
    assert!(
        !qc.lines().any(|l| l.trim_start().starts_with("$include ")),
        "{qc}"
    );
    assert!(!qc.contains("$evil"), "{qc}");
}

#[test]
fn empty_ir_gives_minimal_qc() {
    let files = decompile_files(&ModelIr::default(), QcOptions::default());
    let qc = &files.last().unwrap().text;
    assert!(qc.starts_with("$modelname \"\"\n"), "{qc}");
}
