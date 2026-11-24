use crate::source::matches;
use crate::ServiceError;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// The QC files the arguments name (files, folders, `dir/*.qc`
/// wildcards), sorted and deduplicated, plus one error per argument that
/// names nothing.
pub fn qc_files(args: &[String], recursive: bool) -> (Vec<PathBuf>, Vec<(String, ServiceError)>) {
    let (mut files, mut bad) = (Vec::new(), Vec::new());
    for arg in args {
        match one(Path::new(arg), recursive) {
            Ok(found) if found.is_empty() => {
                let e = ServiceError::NotFound(format!("no .qc matches {arg}"));
                bad.push((arg.clone(), e));
            }
            Ok(found) => files.extend(found),
            Err(e) => bad.push((arg.clone(), e)),
        }
    }
    files.sort();
    files.dedup();
    (files, bad)
}

fn one(path: &Path, recursive: bool) -> Result<Vec<PathBuf>, ServiceError> {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let (root, pattern) = if name.contains(['*', '?']) {
        let parent = path.parent().filter(|p| !p.as_os_str().is_empty());
        (parent.unwrap_or(Path::new(".")), name)
    } else if path.is_dir() {
        (path, "*.qc")
    } else if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    } else {
        return Err(ServiceError::NotFound(path.display().to_string()));
    };
    let depth = if recursive { usize::MAX } else { 1 };
    Ok(WalkDir::new(root)
        .max_depth(depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_str().is_some_and(|n| matches(pattern, n)))
        .map(|e| e.into_path())
        .collect())
}
