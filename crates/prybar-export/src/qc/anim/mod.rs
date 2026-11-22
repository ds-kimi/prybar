//! Pose parameters, weight lists, animations and sequences.

mod animation;
mod labels;
mod poseparam;
mod seq_layers;
mod seq_opts;
mod sequence;
mod weights;

use super::Names;
use prybar_mdl::ModelIr;

pub fn write(ir: &ModelIr, names: &Names) -> String {
    let mut s = String::new();
    poseparam::write(&mut s, ir);
    let weights = weights::write(&mut s, ir);
    let labels = labels::labels(ir);
    for (i, anim) in ir.animations.iter().enumerate() {
        let file = names.anims.get(i).and_then(Option::as_deref);
        animation::write(&mut s, anim, &labels[i], file);
    }
    let cx = sequence::Ctx {
        ir,
        labels: &labels,
    };
    for (seq, w) in ir.sequences.iter().zip(&weights) {
        sequence::write(&mut s, &cx, seq, w.as_deref());
    }
    s
}
