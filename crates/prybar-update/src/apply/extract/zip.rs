use super::{bad, MAX_UNPACKED};
use crate::UpdateError;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Unpacks a zip; `enclosed_name` drops entries that escape `dest`.
pub fn unzip(archive: &Path, dest: &Path) -> Result<(), UpdateError> {
    let mut zip = ::zip::ZipArchive::new(File::open(archive)?).map_err(bad)?;
    let mut total = 0u64;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).map_err(bad)?;
        let rel = file
            .enclosed_name()
            .ok_or_else(|| UpdateError::BadArchive("entry outside archive root".into()))?;
        // ditto puts extended attributes under __MACOSX; the code signature
        // lives in the bundle itself, so they are not needed.
        if rel.starts_with("__MACOSX") {
            continue;
        }
        let out = dest.join(rel);
        if file.is_dir() {
            std::fs::create_dir_all(&out)?;
            continue;
        }
        total += write_entry(&mut file, &out, total)?;
        set_mode(&out, file.unix_mode())?;
    }
    Ok(())
}

/// Writes one entry within what is left of the unpack budget.
fn write_entry(file: &mut impl Read, out: &Path, total: u64) -> Result<u64, UpdateError> {
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut w = File::create(out)?;
    let left = MAX_UNPACKED - total.min(MAX_UNPACKED);
    let n = std::io::copy(&mut file.take(left), &mut w)?;
    if total + n >= MAX_UNPACKED {
        return Err(UpdateError::BadArchive("archive too large".into()));
    }
    Ok(n)
}

#[cfg(unix)]
fn set_mode(path: &Path, mode: Option<u32>) -> Result<(), UpdateError> {
    use std::os::unix::fs::PermissionsExt;
    if let Some(m) = mode {
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(m & 0o777))?;
    }
    Ok(())
}

#[cfg(not(unix))]
fn set_mode(_: &Path, _: Option<u32>) -> Result<(), UpdateError> {
    Ok(())
}
