use super::packed::{mdl_stem, packed};
use super::wildcard::has_wildcard;
use super::{matches, ModelRef};
use crate::package::Package;
use crate::{Result, ServiceError};
use std::path::Path;
use walkdir::WalkDir;

/// The models one argument names. `stems` drop the `.mdl` so companions
/// can be found by appending their suffix.
#[derive(Debug)]
pub struct Batch {
    pub package: Option<Package>,
    pub stems: Vec<String>,
}

pub fn expand(arg: &str, recursive: bool) -> Result<Batch> {
    let stems = match ModelRef::parse(arg) {
        ModelRef::Path(p) => disk(&p, recursive)?,
        ModelRef::Packed { package, inner } => {
            let package = Package::open(&package)?;
            let stems = packed(&package, &inner, recursive);
            return non_empty(arg, Some(package), stems);
        }
    };
    non_empty(arg, None, stems)
}

fn non_empty(arg: &str, package: Option<Package>, stems: Vec<String>) -> Result<Batch> {
    if stems.is_empty() {
        return Err(ServiceError::NotFound(format!("no .mdl matches {arg}")));
    }
    Ok(Batch { package, stems })
}

fn disk(path: &Path, recursive: bool) -> Result<Vec<String>> {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let (root, pattern) = if has_wildcard(name) {
        let parent = path.parent().filter(|p| !p.as_os_str().is_empty());
        (
            parent.unwrap_or(Path::new(".")).to_path_buf(),
            name.to_owned(),
        )
    } else if path.is_dir() {
        (path.to_path_buf(), "*.mdl".to_owned())
    } else if path.is_file() {
        return Ok(stem(path).into_iter().collect());
    } else {
        return Err(ServiceError::NotFound(path.display().to_string()));
    };
    let depth = if recursive { usize::MAX } else { 1 };
    let mut out: Vec<String> = WalkDir::new(&root)
        .max_depth(depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_str().is_some_and(|n| matches(&pattern, n)))
        .filter_map(|e| stem(e.path()))
        .collect();
    out.sort();
    Ok(out)
}

fn stem(path: &Path) -> Option<String> {
    mdl_stem(path.to_str()?).map(str::to_owned)
}
