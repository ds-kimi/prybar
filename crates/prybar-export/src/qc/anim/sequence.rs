use super::{seq_layers, seq_opts};
use crate::qc::fmt::{num, quote};
use prybar_mdl::ir::Sequence;
use prybar_mdl::ModelIr;
use std::fmt::Write;

pub struct Ctx<'a> {
    pub ir: &'a ModelIr,
    pub labels: &'a [String],
}

pub fn write(s: &mut String, cx: &Ctx, seq: &Sequence, weights: Option<&str>) {
    // Same rule as `$animation`: a brace on its own line is not read.
    let _ = writeln!(s, "$sequence {} {{", quote(&seq.name));
    anims(s, cx, seq);
    seq_opts::write(s, cx.ir, seq);
    seq_layers::write(s, cx, seq);
    if let Some(w) = weights {
        let _ = writeln!(s, "  weightlist {}", quote(w));
    }
    s.push_str("}\n");
}

/// The blend grid row-major, then one `blend` per axis that varies.
fn anims(s: &mut String, cx: &Ctx, seq: &Sequence) {
    s.push(' ');
    for &a in &seq.anims {
        let label = usize::try_from(a).ok().and_then(|a| cx.labels.get(a));
        let _ = write!(s, " {}", quote(label.map_or("", String::as_str)));
    }
    s.push('\n');
    let [w, h] = seq.group_size;
    if w > 1 && h > 1 {
        let _ = writeln!(s, "  blendwidth {w}");
    }
    for axis in 0..2 {
        let param = usize::try_from(seq.param_index[axis]).ok();
        let param = param.and_then(|p| cx.ir.pose_params.get(p));
        if let Some(p) = param.filter(|_| seq.group_size[axis] > 1) {
            let (a, b) = (num(seq.param_start[axis]), num(seq.param_end[axis]));
            let _ = writeln!(s, "  blend {} {a} {b}", quote(&p.name));
        }
    }
}

/// First animation of the sequence, the one events and layers are timed
/// against.
pub fn frames(cx: &Ctx, seq: &Sequence) -> usize {
    let a = seq.anims.first().and_then(|&a| usize::try_from(a).ok());
    a.and_then(|a| cx.ir.animations.get(a))
        .map_or(1, |a| a.frame_count)
}
