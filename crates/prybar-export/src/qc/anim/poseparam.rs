use crate::qc::fmt::{num, quote};
use prybar_mdl::ModelIr;
use std::fmt::Write;

pub fn write(s: &mut String, ir: &ModelIr) {
    for p in &ir.pose_params {
        let _ = write!(
            s,
            "$poseparameter {} {} {}",
            quote(&p.name),
            num(p.start),
            num(p.end)
        );
        if p.loop_range != 0.0 {
            let _ = write!(s, " loop {}", num(p.loop_range));
        }
        s.push('\n');
    }
}
