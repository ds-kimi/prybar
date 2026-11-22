use super::walk::walk;
use crate::ServiceError;
use std::path::Path;

// Each format crate wraps I/O failures in its own `Io` variant; they are
// pulled out so a full disk exits 5 (I/O), not 4 (malformed package).

pub fn vpk(at: &Path, e: prybar_vpk::Error) -> ServiceError {
    match e {
        prybar_vpk::Error::Io(io) => ServiceError::io(at, io),
        prybar_vpk::Error::Walk(w) => walk(at, w),
        e => ServiceError::invalid(at.display(), e),
    }
}

pub fn gma(at: &Path, e: prybar_gma::Error) -> ServiceError {
    match e {
        prybar_gma::Error::Io(io) => ServiceError::io(at, io),
        prybar_gma::Error::Walk(w) => walk(at, w),
        e => ServiceError::invalid(at.display(), e),
    }
}

pub fn apk(at: &Path, e: prybar_apk::Error) -> ServiceError {
    match e {
        prybar_apk::Error::Io(io) => ServiceError::io(at, io),
        e => ServiceError::invalid(at.display(), e),
    }
}

pub fn hfs(at: &Path, e: prybar_hfs::Error) -> ServiceError {
    match e {
        prybar_hfs::Error::Io(io) => ServiceError::io(at, io),
        e => ServiceError::invalid(at.display(), e),
    }
}
