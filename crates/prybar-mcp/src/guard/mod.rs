//! Argument checks the JSON schema cannot express. Agents run in some
//! unknown working directory, so a relative path would silently point
//! somewhere else: only absolute paths are accepted.

use prybar_service::{Result, ServiceError};
use std::path::{Path, PathBuf};

/// Longer than any real path on the supported OSes; stops a runaway
/// argument before it reaches the filesystem.
const MAX_PATH_BYTES: usize = 4096;

pub fn absolute(field: &str, value: &str) -> Result<PathBuf> {
    if value.is_empty() || value.len() > MAX_PATH_BYTES || value.contains('\0') {
        return Err(ServiceError::Usage(format!(
            "{field}: expected a path of 1 to {MAX_PATH_BYTES} bytes"
        )));
    }
    if !Path::new(value).is_absolute() || !local(Path::new(value)) {
        return Err(ServiceError::Usage(format!(
            "{field}: `{value}` is not an absolute local path"
        )));
    }
    Ok(PathBuf::from(value))
}

/// Windows UNC and device paths are refused: opening `\\host\share`
/// sends the user's NTLM credentials to that host, and an agent can be
/// talked into passing one. Only drive-letter paths are local.
fn local(path: &Path) -> bool {
    use std::path::{Component, Prefix};
    match path.components().next() {
        Some(Component::Prefix(p)) => matches!(p.kind(), Prefix::Disk(_) | Prefix::VerbatimDisk(_)),
        _ => !cfg!(windows),
    }
}

/// Model arguments may carry `package:inner` or wildcards; only the start
/// has to be an absolute path, which `Path::is_absolute` checks on the
/// whole string.
pub fn model_arg(field: &str, value: &str) -> Result<String> {
    absolute(field, value).map(|_| value.to_owned())
}

/// Caps a list an agent asked for, keeping a server-side ceiling so one
/// call cannot flood the context window.
pub fn limit(asked: Option<usize>, default: usize, ceiling: usize) -> usize {
    asked.unwrap_or(default).clamp(1, ceiling)
}
