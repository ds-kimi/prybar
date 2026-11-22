//! Bones, attachments, hitboxes and procedural bones.

mod attach;
mod bones;
mod hbox;
pub(crate) mod ik;
mod jiggle;
mod jiggle_keys;

use super::fmt::quote;
use super::Names;
use prybar_mdl::ModelIr;
use std::fmt::Write;

pub fn write(ir: &ModelIr, names: &Names) -> String {
    let mut s = String::new();
    bones::write(&mut s, ir);
    attach::write(&mut s, ir);
    hbox::write(&mut s, ir);
    ik::write(&mut s, ir);
    jiggle::write(&mut s, ir);
    if let Some(vrd) = &names.vrd {
        let _ = writeln!(s, "$proceduralbones {}", quote(vrd));
    }
    s
}
