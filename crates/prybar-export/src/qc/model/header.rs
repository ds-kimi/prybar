use crate::qc::fmt::{num, quote, vec3};
use crate::qc::sanitize::{balanced, game_path};
use prybar_mdl::ModelIr;
use std::fmt::Write;

/// `STUDIOHDR_FLAGS_*` bits that come from a QC command; the rest are
/// computed by studiomdl and would be wrong to force.
const FLAG_COMMANDS: [(i32, &str); 9] = [
    (0x4, "$opaque"),
    (0x8, "$mostlyopaque"),
    (0x10, "$staticprop"),
    (0x200, "$obsolete"),
    (0x800, "$noforcedfade"),
    (0x1000, "$forcephonemecrossfade"),
    (0x10000, "$ambientboost"),
    (0x20000, "$donotcastshadows"),
    (0x40000, "$casttextureshadows"),
];

pub fn write(s: &mut String, ir: &ModelIr) {
    let _ = writeln!(s, "$modelname {}", game_path(&ir.name));
    for dir in &ir.material_dirs {
        let _ = writeln!(s, "$cdmaterials {}", game_path(dir));
    }
    if !ir.surface_prop.is_empty() {
        let _ = writeln!(s, "$surfaceprop {}", quote(&ir.surface_prop));
    }
    for (bit, cmd) in FLAG_COMMANDS {
        if ir.flags & bit != 0 {
            let _ = writeln!(s, "{cmd}");
        }
    }
    let _ = writeln!(s, "$illumposition {}", vec3(ir.illum_position));
    let _ = writeln!(s, "$eyeposition {}", vec3(ir.eye_position));
    boxes(s, ir);
    // Only MDL 2531 stores the speech crossfade window.
    if ir.phoneme_filter != [0.0; 2] {
        let [min, max] = ir.phoneme_filter;
        let _ = writeln!(s, "$phonemefilter {} {}", num(min), num(max));
    }
    for m in &ir.include_models {
        let _ = writeln!(s, "$includemodel {}", game_path(&m.path));
    }
    key_values(s, &ir.key_values);
}

/// Zero boxes mean "let studiomdl compute it", so they are not written.
fn boxes(s: &mut String, ir: &ModelIr) {
    let pairs = [
        ("$bbox", ir.view_min, ir.view_max),
        ("$cbox", ir.hull_min, ir.hull_max),
    ];
    for (cmd, min, max) in pairs {
        if min != [0.0; 3] || max != [0.0; 3] {
            let _ = writeln!(s, "{cmd} {} {}", vec3(min), vec3(max));
        }
    }
}

/// The MDL stores the block body as text, copied back verbatim.
fn key_values(s: &mut String, text: &str) {
    let text = text.trim_matches(|c: char| c.is_whitespace() || c == '\0');
    if text.is_empty() || !balanced(text) {
        return;
    }
    let _ = writeln!(s, "$keyvalues\n{{\n{text}\n}}");
}
