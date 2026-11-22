use crate::qc::fmt::{quote, vec3};
use prybar_mdl::ModelIr;
use std::fmt::Write;

pub fn write(s: &mut String, ir: &ModelIr) {
    for set in &ir.hitbox_sets {
        let _ = writeln!(s, "$hboxset {}", quote(&set.name));
        for h in &set.hitboxes {
            let bone = usize::try_from(h.bone).ok().and_then(|b| ir.bones.get(b));
            let _ = writeln!(
                s,
                "$hbox {} {} {} {}",
                h.group,
                quote(bone.map_or("", |b| b.name.as_str())),
                vec3(h.min),
                vec3(h.max),
            );
            if !h.name.is_empty() {
                let _ = write!(s, " {}", quote(&h.name));
            }
            s.push('\n');
        }
    }
}
