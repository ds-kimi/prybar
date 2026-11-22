use crate::package::{classify, Format};
use crate::{Result, ServiceError};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackRequest {
    pub src: PathBuf,
    /// The extension picks the format (`.vpk` or `.gma`).
    pub out: PathBuf,
    /// VPK only: 1 or 2.
    pub vpk_version: u32,
    /// VPK only: split data into `_NNN.vpk` archives of about this size.
    pub chunk_size: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PackReport {
    pub format: Format,
    pub out: PathBuf,
    pub files: usize,
}

pub fn pack(req: &PackRequest) -> Result<PackReport> {
    if !matches!(req.vpk_version, 1 | 2) {
        let v = req.vpk_version;
        return Err(ServiceError::Usage(format!(
            "VPK version {v} is not 1 or 2"
        )));
    }
    if !req.src.is_dir() {
        return Err(ServiceError::NotFound(req.src.display().to_string()));
    }
    let format = match Format::from_path(&req.out) {
        Some(f @ (Format::Vpk | Format::Gma)) => f,
        _ => return Err(ServiceError::Usage(unsupported(&req.out))),
    };
    let files = match format {
        Format::Vpk => vpk(req).map_err(|e| classify::vpk(&req.out, e))?,
        _ => gma(req)?,
    };
    Ok(PackReport {
        format,
        out: req.out.clone(),
        files,
    })
}

fn vpk(req: &PackRequest) -> prybar_vpk::Result<usize> {
    let opts = prybar_vpk::PackOptions {
        version: req.vpk_version,
        chunk_size: req.chunk_size,
    };
    prybar_vpk::pack(&req.src, &req.out, &opts)
}

/// gmad needs `addon.json` at the root; without this check the missing
/// file surfaces as an io error on the output path, which misleads.
fn gma(req: &PackRequest) -> Result<usize> {
    if !req.src.join("addon.json").is_file() {
        return Err(ServiceError::Usage(format!(
            "{}: a .gma needs an addon.json in the source folder",
            req.src.display()
        )));
    }
    prybar_gma::pack(&req.src, &req.out).map_err(|e| classify::gma(&req.out, e))
}

fn unsupported(out: &Path) -> String {
    format!(
        "{}: can only pack .vpk or .gma (APK and HFS are read-only)",
        out.display()
    )
}
