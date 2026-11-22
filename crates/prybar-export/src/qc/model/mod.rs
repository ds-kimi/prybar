//! Header, geometry and skin commands.

pub(crate) mod body;
mod header;
mod lod;
mod skins;

use super::Names;
pub use body::is_flexed;
use prybar_mdl::ModelIr;

pub fn write(ir: &ModelIr, names: &Names) -> String {
    let mut s = String::new();
    header::write(&mut s, ir);
    body::write(&mut s, ir, names);
    lod::write(&mut s, ir, names);
    skins::write(&mut s, ir);
    s
}
