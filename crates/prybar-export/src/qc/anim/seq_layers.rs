use super::sequence::{frames, Ctx};
use crate::qc::fmt::{num, quote};
use crate::qc::sanitize::ident;
use prybar_mdl::ir::{AutoLayer, Sequence};
use std::fmt::Write;

/// `STUDIO_AL_*` bits that map to `blendlayer` options.
const LAYER_FLAGS: [(i32, &str); 4] = [
    (0x40, "spline"),
    (0x80, "xfade"),
    (0x200, "noblend"),
    (0x1000, "local"),
];
const STUDIO_AL_POSE: i32 = 0x4000;

/// Events and layers store cycles; the QC wants frames of the first anim.
pub fn write(s: &mut String, cx: &Ctx, seq: &Sequence) {
    let last = frames(cx, seq).saturating_sub(1) as f32;
    for e in &seq.events {
        let id = if e.name.is_empty() {
            e.event.to_string()
        } else {
            ident(&e.name)
        };
        let frame = num((e.cycle * last).round());
        let _ = writeln!(s, "  {{ event {id} {frame} {} }}", quote(&e.options));
    }
    for layer in &seq.auto_layers {
        let target = usize::try_from(layer.sequence)
            .ok()
            .and_then(|i| cx.ir.sequences.get(i));
        let name = quote(target.map_or("", |t| t.name.as_str()));
        if layer.flags == 0 && layer.ramp == [0.0; 4] {
            let _ = writeln!(s, "  addlayer {name}");
        } else {
            blend_layer(s, cx, layer, &name, last);
        }
    }
}

fn blend_layer(s: &mut String, cx: &Ctx, layer: &AutoLayer, name: &str, last: f32) {
    let _ = write!(s, "  blendlayer {name}");
    // Pose-driven layers ramp over pose values, not cycles.
    let pose_ramp = layer.flags & STUDIO_AL_POSE != 0;
    for r in layer.ramp {
        let v = if pose_ramp { r } else { (r * last).round() };
        let _ = write!(s, " {}", num(v));
    }
    for (bit, opt) in LAYER_FLAGS {
        if layer.flags & bit != 0 {
            let _ = write!(s, " {opt}");
        }
    }
    let pose = usize::try_from(layer.pose)
        .ok()
        .and_then(|p| cx.ir.pose_params.get(p));
    if let Some(p) = pose.filter(|_| layer.flags & STUDIO_AL_POSE != 0) {
        let _ = write!(s, " poseparameter {}", quote(&p.name));
    }
    s.push('\n');
}
