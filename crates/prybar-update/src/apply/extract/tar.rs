use super::{bad, MAX_UNPACKED};
use crate::UpdateError;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Unpacks a gzipped tarball; `tar` itself refuses `..` and absolute paths.
pub fn untar(archive: &Path, dest: &Path) -> Result<(), UpdateError> {
    let gz = flate2::read::GzDecoder::new(File::open(archive)?);
    let mut tar = ::tar::Archive::new(gz.take(MAX_UNPACKED));
    tar.set_preserve_permissions(true);
    for entry in tar.entries().map_err(bad)? {
        let mut entry = entry.map_err(bad)?;
        // unpack_in refuses `..` and absolute paths and returns false.
        if !entry.unpack_in(dest).map_err(bad)? {
            return Err(UpdateError::BadArchive("entry outside archive root".into()));
        }
    }
    Ok(())
}
