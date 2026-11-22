use crate::qc::fmt::{num, quote};
use crate::qc::sanitize::balanced;
use crate::qc::skeleton::ik::lock_args;
use prybar_mdl::ir::Sequence;
use prybar_mdl::ModelIr;
use std::fmt::Write;

const FLAGS: [(i32, &str); 6] = [
    (0x1, "loop"),
    (0x2, "snap"),
    (0x8, "autoplay"),
    (0x100, "realtime"),
    (0x400, "hidden"),
    (0x4000, "worldspace"),
];
const STUDIO_CYCLEPOSE: i32 = 0x80;

pub fn write(s: &mut String, ir: &ModelIr, seq: &Sequence) {
    for (bit, name) in FLAGS {
        if seq.flags & bit != 0 {
            let _ = writeln!(s, "  {name}");
        }
    }
    if let Some(delta) = delta_option(seq.flags) {
        let _ = writeln!(s, "  {delta}");
    }
    if !seq.activity_name.is_empty() {
        let _ = writeln!(
            s,
            "  activity {} {}",
            quote(&seq.activity_name),
            seq.activity_weight
        );
    }
    for m in &seq.activity_modifiers {
        let _ = writeln!(s, "  activitymodifier {}", quote(m));
    }
    let _ = writeln!(
        s,
        "  fadein {}\n  fadeout {}",
        num(seq.fade_in),
        num(seq.fade_out)
    );
    node(s, seq);
    extras(s, ir, seq);
}

fn extras(s: &mut String, ir: &ModelIr, seq: &Sequence) {
    let pose = usize::try_from(seq.cycle_pose)
        .ok()
        .and_then(|p| ir.pose_params.get(p));
    if let Some(p) = pose.filter(|_| seq.flags & STUDIO_CYCLEPOSE != 0) {
        let _ = writeln!(s, "  posecycle {}", quote(&p.name));
    }
    for lock in &seq.ik_locks {
        if let Some(args) = lock_args(ir, lock) {
            let _ = writeln!(s, "  iklock {args}");
        }
    }
    let kv = seq
        .key_values
        .trim_matches(|c: char| c.is_whitespace() || c == '\0');
    if !kv.is_empty() && balanced(kv) {
        let _ = writeln!(s, "  keyvalues\n  {{\n{kv}\n  }}");
    }
}

/// studiomdl's `delta` sets STUDIO_DELTA and STUDIO_POST, `predelta`
/// only STUDIO_DELTA; POST alone has no QC option.
pub fn delta_option(flags: i32) -> Option<&'static str> {
    match flags & 0x14 {
        0x14 => Some("delta"),
        0x4 => Some("predelta"),
        _ => None,
    }
}

/// Node 0 means "no transition graph entry".
fn node(s: &mut String, seq: &Sequence) {
    let (entry, exit) = (seq.entry_node, seq.exit_node);
    if entry == 0 && exit == 0 {
        return;
    }
    let _ = if entry == exit {
        writeln!(s, "  node \"{entry}\"")
    } else if seq.node_flags & 1 != 0 {
        writeln!(s, "  rtransition \"{entry}\" \"{exit}\"")
    } else {
        writeln!(s, "  transition \"{entry}\" \"{exit}\"")
    };
}
