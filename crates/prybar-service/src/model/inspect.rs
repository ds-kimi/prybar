use crate::package::Package;
use crate::source::{ModelRef, Origin};
use crate::{Result, ServiceError};
use prybar_mdl::Summary;

/// Summary of one model, on disk or inside a package.
pub fn inspect(arg: &str) -> Result<Summary> {
    let data = match ModelRef::parse(arg) {
        ModelRef::Path(p) => std::fs::read(&p).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => ServiceError::NotFound(p.display().to_string()),
            _ => ServiceError::io(&p, e),
        })?,
        ModelRef::Packed { package, inner } => {
            let pkg = Package::open(&package)?;
            Origin::Package(&pkg)
                .get(&inner, "")?
                .ok_or_else(|| ServiceError::NotFound(arg.to_owned()))?
        }
    };
    prybar_mdl::inspect(&data).map_err(|e| ServiceError::invalid(arg, e))
}
