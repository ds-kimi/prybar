use crate::qc::fmt::{num, quote};
use prybar_mdl::ir::Animation;
use std::fmt::Write;

const STUDIO_LOOPING: i32 = 0x1;
/// `STUDIO_*` motion bits in the order `walkframe` lists them.
const MOTION: [(i32, &str); 12] = [
    (0x1, "X"),
    (0x2, "Y"),
    (0x4, "Z"),
    (0x8, "XR"),
    (0x10, "YR"),
    (0x20, "ZR"),
    (0x40, "LX"),
    (0x80, "LY"),
    (0x100, "LZ"),
    (0x200, "LXR"),
    (0x400, "LYR"),
    (0x800, "LZR"),
];

/// Anims whose frames sit in a missing `.ani` get a comment instead, so
/// the gap is visible rather than silently dropped.
pub fn write(s: &mut String, anim: &Animation, label: &str, file: Option<&str>) {
    let Some(file) = file else {
        let _ = writeln!(s, "// {label}: no decoded frames (missing .ani?)");
        return;
    };
    // studiomdl rejects the option block of `$animation` unless its brace
    // sits on the command line itself.
    let _ = writeln!(s, "$animation {} {} {{", quote(label), quote(file));
    let _ = writeln!(s, "  fps {}", num(anim.fps));
    if anim.flags & STUDIO_LOOPING != 0 {
        s.push_str("  loop\n");
    }
    // `predelta` is a `$sequence`-only option; studiomdl aborts on it
    // here, and the sequence that plays this anim writes it instead.
    if let Some("delta") = super::seq_opts::delta_option(anim.flags) {
        s.push_str("  delta\n");
    }
    for m in &anim.movements {
        let _ = write!(s, "  walkframe {}", m.end_frame);
        for (bit, name) in MOTION {
            if m.motion_flags & bit != 0 {
                let _ = write!(s, " {name}");
            }
        }
        s.push('\n');
    }
    s.push_str("}\n");
}
