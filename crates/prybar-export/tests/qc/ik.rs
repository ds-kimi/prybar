use prybar_export::qc::{qc_files, Names, QcOptions};
use prybar_mdl::ir::{IkChain, IkLock};
use prybar_mdl::ModelIr;

fn with_chains() -> ModelIr {
    let mut ir = super::fixture::ir();
    let chain = |name: &str, bone| IkChain {
        name: name.into(),
        bone,
        knee: [0.707, 0.707, 0.0],
    };
    ir.ik_chains = vec![chain("rhand", 1), chain("broken", 999)];
    let lock = |chain| IkLock {
        chain,
        pos_weight: 1.0,
        local_q_weight: 0.5,
        flags: 0,
    };
    ir.ik_auto_locks = vec![lock(0), lock(7)];
    ir.sequences[0].ik_locks = vec![lock(0), lock(-1)];
    ir
}

fn qc(ir: &ModelIr) -> String {
    qc_files(ir, &Names::new(ir), QcOptions::default())
        .remove(0)
        .text
}

/// Crowbar issue #29 and the "unknown bone in ikrule" Steam threads:
/// chains must be written, before any sequence refers to them.
#[test]
fn writes_chains_and_locks_before_sequences() {
    let ir = with_chains();
    let text = qc(&ir);
    let bone = &ir.bones[1].name;
    let line = format!("$ikchain \"rhand\" \"{bone}\" knee 0.707 0.707 0");
    let chain_at = text.find(&line).expect(&text);
    assert!(chain_at < text.find("$sequence").unwrap());
    assert!(text.contains("$ikautoplaylock \"rhand\" 1 0.5"));
    assert!(text.contains("  iklock \"rhand\" 1 0.5"));
}

/// Out-of-range chain and bone indices from a crafted file are dropped.
#[test]
fn drops_dangling_chain_refs() {
    let text = qc(&with_chains());
    assert!(!text.contains("broken"));
    assert_eq!(text.matches("ikautoplaylock").count(), 1);
    assert_eq!(text.matches("iklock").count(), 1);
}
