use super::{classify, Format, Kind};
use crate::{Result, ServiceError};
use std::path::Path;

pub fn open(path: &Path) -> Result<Kind> {
    let format = Format::from_path(path).ok_or_else(|| {
        ServiceError::Usage(format!(
            "{}: not a package (expected .vpk, .gma, .apk or .hfs)",
            path.display()
        ))
    })?;
    if !path.is_file() {
        return Err(ServiceError::NotFound(path.display().to_string()));
    }
    Ok(match format {
        Format::Vpk => Kind::Vpk(prybar_vpk::Vpk::open(path).map_err(|e| classify::vpk(path, e))?),
        Format::Gma => Kind::Gma(prybar_gma::Gma::open(path).map_err(|e| classify::gma(path, e))?),
        Format::Apk => Kind::Apk(prybar_apk::Apk::open(path).map_err(|e| classify::apk(path, e))?),
        Format::Hfs => Kind::Hfs(prybar_hfs::Hfs::open(path).map_err(|e| classify::hfs(path, e))?),
    })
}
