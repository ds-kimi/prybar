use super::jiggle_keys::{Key, BASE, BOING, FLEXIBLE, RIGID};
use crate::qc::fmt::{num, quote};
use prybar_mdl::ir::{Jiggle, ProcBone};
use prybar_mdl::ModelIr;
use std::fmt::Write;

const IS_FLEXIBLE: i32 = 0x01;
const IS_RIGID: i32 = 0x02;
const HAS_BASE_SPRING: i32 = 0x40;
const HAS_LENGTH_CONSTRAINT: i32 = 0x20;
const IS_BOING: i32 = 0x80;

pub fn write(s: &mut String, ir: &ModelIr) {
    for bone in &ir.bones {
        if let Some(ProcBone::Jiggle(j)) = &bone.procedural {
            let _ = writeln!(s, "$jigglebone {} {{", quote(&bone.name));
            let blocks = [
                (IS_FLEXIBLE, "is_flexible", FLEXIBLE),
                (IS_RIGID, "is_rigid", RIGID),
                (HAS_BASE_SPRING, "has_base_spring", BASE),
                (IS_BOING, "is_boing", BOING),
            ];
            for (bit, name, keys) in blocks {
                if j.flags & bit != 0 {
                    block(s, j, name, keys);
                }
            }
            s.push_str("}\n");
        }
    }
}

/// Short `params` (hostile or truncated files) read as zeros.
fn block(s: &mut String, j: &Jiggle, name: &str, keys: &[Key]) {
    let _ = writeln!(s, "  {name} {{");
    let at = |i: usize| j.params.get(i).copied().unwrap_or(0.0);
    for key in keys {
        if key.flag != 0 && j.flags & key.flag == 0 {
            continue;
        }
        let _ = write!(s, "    {}", key.name);
        for &i in key.params {
            let _ = write!(s, " {}", num((key.convert)(at(i))));
        }
        s.push('\n');
    }
    // studiomdl constrains a flexible bone's length unless told otherwise.
    if name == "is_flexible" && j.flags & HAS_LENGTH_CONSTRAINT == 0 {
        s.push_str("    allow_length_flex\n");
    }
    s.push_str("  }\n");
}
