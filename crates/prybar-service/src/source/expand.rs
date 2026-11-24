use super::packed::{mdl_stem, packed, packed_root};
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
    /// Folder the stems are relative to in the mirrored output tree:
    /// the input folder, the wildcard base, or the package prefix.
    pub root: String,
}

pub fn expand(arg: &str, recursive: bool) -> Result<Batch> {
    let (package, stems, root) = match ModelRef::parse(arg) {
        ModelRef::Path(p) => {
            let (stems, root) = disk(&p, recursive)?;
            (None, stems, root)
        }
        ModelRef::Packed { package, inner } => {
            let package = Package::open(&package)?;
            let stems = packed(&package, &inner, recursive);
            (Some(package), stems, packed_root(&inner))
        }
    };
    if stems.is_empty() {
        return Err(ServiceError::NotFound(format!("no .mdl matches {arg}")));
    }
    Ok(Batch {
        package,
        stems,
        root,
    })
}

fn disk(path: &Path, recursive: bool) -> Result<(Vec<String>, String)> {
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
        let parent = path.parent().and_then(Path::to_str).unwrap_or("");
        return Ok((stem(path).into_iter().collect(), parent.to_owned()));
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
    Ok((out, root.to_str().unwrap_or("").to_owned()))
}

fn stem(path: &Path) -> Option<String> {
    mdl_stem(path.to_str()?).map(str::to_owned)
}
