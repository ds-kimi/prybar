use prybar_mdl::ir::Bone;
use std::fmt::Write;

/// Writes `nodes` and the bind pose in `skeleton`; an empty bone list still
/// gets one root node because studiomdl rejects an SMD without bones.
pub fn write(s: &mut String, bones: &[Bone]) {
    nodes(s, bones);
    s.push_str("skeleton\ntime 0\n");
    if bones.is_empty() {
        s.push_str("  0 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000\n");
    }
    for (i, b) in bones.iter().enumerate() {
        bone_line(s, i, b.position, b.rotation);
    }
    s.push_str("end\n");
}

pub fn nodes(s: &mut String, bones: &[Bone]) {
    s.push_str("nodes\n");
    if bones.is_empty() {
        s.push_str("  0 \"root\" -1\n");
    }
    for (i, b) in bones.iter().enumerate() {
        let _ = writeln!(s, "  {i} \"{}\" {}", b.name, b.parent);
    }
    s.push_str("end\n");
}

/// Values that print as zero are written as `0.000000`: rotations and
/// float noise often give `-0.000000`, which makes diffs noisy.
pub fn bone_line(s: &mut String, i: usize, pos: [f32; 3], rot: [f32; 3]) {
    let tidy = |v: [f32; 3]| v.map(|x| if x.abs() < 5e-7 { 0.0 } else { x });
    let ([px, py, pz], [rx, ry, rz]) = (tidy(pos), tidy(rot));
    let _ = writeln!(s, "  {i} {px:.6} {py:.6} {pz:.6} {rx:.6} {ry:.6} {rz:.6}");
}
