use super::load;
use crate::package::Package;
use crate::source::{ModelRef, Origin};
use crate::{Result, ServiceError};
use prybar_mdl::ModelIr;

/// Full model with its meshes, for previews: on disk or inside a package.
pub fn open(arg: &str) -> Result<ModelIr> {
    match ModelRef::parse(arg) {
        ModelRef::Path(p) => {
            let text = p.to_string_lossy();
            let stem = stem(&text).ok_or_else(|| ServiceError::NotFound(text.to_string()))?;
            load(Origin::Disk, stem)
        }
        ModelRef::Packed { package, inner } => {
            let pkg = Package::open(&package)?;
            let stem = stem(&inner).ok_or_else(|| ServiceError::NotFound(arg.to_owned()))?;
            load(Origin::Package(&pkg), stem)
        }
    }
}

/// The path without its `.mdl` extension, any case.
fn stem(path: &str) -> Option<&str> {
    let cut = path.len().checked_sub(4)?;
    let ext = path.get(cut..)?;
    ext.eq_ignore_ascii_case(".mdl").then(|| &path[..cut])
}
