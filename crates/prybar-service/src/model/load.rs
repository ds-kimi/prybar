use super::{geometry, goldsrc};
use crate::source::Origin;
use crate::{Result, ServiceError};
use prybar_mdl::{Family, ModelIr};

/// Parses `<stem>.mdl` with every companion found next to it. Broken or
/// missing companions become notes, not failures: the skeleton and QC
/// are still worth writing.
pub fn load(origin: Origin, stem: &str) -> Result<ModelIr> {
    let mdl = origin
        .get(stem, ".mdl")?
        .ok_or_else(|| ServiceError::NotFound(format!("{stem}.mdl")))?;
    let bad = |e: prybar_mdl::Error| ServiceError::invalid(format!("{stem}.mdl"), e);
    let version = prybar_mdl::detect(&mdl).map_err(bad)?;
    if version.family == Family::GoldSrc && matches!(version.number, 10 | 14) {
        return goldsrc::load(origin, stem, &mdl).map_err(bad);
    }
    let ani = origin.get(stem, ".ani").unwrap_or(None);
    let mut ir = prybar_mdl::parse_with_ani(&mdl, ani.as_deref()).map_err(bad)?;
    if version.family != Family::SourceLegacy {
        geometry::attach(origin, stem, &mut ir);
    }
    Ok(ir)
}
