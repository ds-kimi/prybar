use crate::ServiceError;
use std::path::Path;

/// Names the file walkdir choked on, not the folder that was asked for.
pub fn walk(at: &Path, w: walkdir::Error) -> ServiceError {
    let at = w.path().unwrap_or(at).to_path_buf();
    match w.into_io_error() {
        Some(io) => ServiceError::io(&at, io),
        None => ServiceError::invalid(at.display(), "folder loop while walking"),
    }
}
