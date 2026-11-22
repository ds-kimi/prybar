mod tar;
mod zip;

use crate::UpdateError;
use std::path::Path;

/// Cap on the unpacked size, so a crafted archive cannot fill the disk.
pub(super) const MAX_UNPACKED: u64 = 1 << 30;

/// Unpacks a `.zip` or `.tar.gz` release archive into `dest`. Entries that
/// would land outside `dest` are refused.
pub fn extract(archive: &Path, dest: &Path) -> Result<(), UpdateError> {
    let name = archive.to_string_lossy();
    if name.ends_with(".tar.gz") {
        tar::untar(archive, dest)
    } else if name.ends_with(".zip") {
        zip::unzip(archive, dest)
    } else {
        Err(UpdateError::BadArchive(format!(
            "unknown archive type: {name}"
        )))
    }
}

pub(super) fn bad(e: impl std::fmt::Display) -> UpdateError {
    UpdateError::BadArchive(e.to_string())
}
