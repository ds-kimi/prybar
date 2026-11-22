use crate::package::{Format, Package};
use crate::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnpackReport {
    pub format: Format,
    pub out: PathBuf,
    pub files: usize,
}

pub fn unpack(path: &Path, out: &Path) -> Result<UnpackReport> {
    let pkg = Package::open(path)?;
    let files = pkg.extract_all(out)?;
    Ok(UnpackReport {
        format: pkg.format(),
        out: out.to_path_buf(),
        files,
    })
}
