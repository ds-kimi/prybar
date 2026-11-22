use crate::{Result, ServiceError};
use prybar_workshop::spec::ItemSpec;
use std::path::Path;

/// A spec is a few hundred bytes; refusing big files keeps a wrong path
/// (a VPK, a log) from being read whole into memory.
const MAX_BYTES: u64 = 1 << 20;

pub fn load_spec(path: &Path) -> Result<ItemSpec> {
    let meta = std::fs::metadata(path).map_err(|e| read_error(path, e))?;
    if meta.len() > MAX_BYTES {
        return Err(ServiceError::invalid(path.display(), "larger than 1 MiB"));
    }
    let text = std::fs::read_to_string(path).map_err(|e| read_error(path, e))?;
    serde_json::from_str(&text).map_err(|e| ServiceError::invalid(path.display(), e))
}

fn read_error(path: &Path, e: std::io::Error) -> ServiceError {
    match e.kind() {
        std::io::ErrorKind::NotFound => ServiceError::NotFound(path.display().to_string()),
        _ => ServiceError::io(path, e),
    }
}
