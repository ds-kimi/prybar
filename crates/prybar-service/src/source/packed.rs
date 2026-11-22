use super::matches;
use super::wildcard::has_wildcard;
use crate::package::Package;

fn depth(path: &str) -> usize {
    path.matches('/').count()
}

/// Model stems inside a package: one entry, a wildcard, or a folder
/// (empty = the whole package).
pub fn packed(package: &Package, inner: &str, recursive: bool) -> Vec<String> {
    let inner = inner.trim_matches('/');
    let folder = format!("{}/", inner.to_ascii_lowercase());
    let wanted = |path: &str| {
        let lower = path.to_ascii_lowercase();
        if has_wildcard(inner) {
            let same_depth = depth(path) == depth(inner);
            return matches(inner, path) && (recursive || same_depth);
        }
        if lower.ends_with(".mdl") && lower == inner.to_ascii_lowercase() {
            return true;
        }
        let rest = match inner.is_empty() {
            true => Some(lower.as_str()),
            false => lower.strip_prefix(&folder),
        };
        rest.is_some_and(|r| recursive || !r.contains('/'))
    };
    let mut out: Vec<String> = package
        .entries()
        .into_iter()
        .filter(|e| wanted(&e.path))
        .filter_map(|e| mdl_stem(&e.path).map(str::to_owned))
        .collect();
    out.sort();
    out
}

/// `get` instead of slicing: entry names are untrusted UTF-8 and a cut
/// inside a multi-byte char would panic.
pub fn mdl_stem(path: &str) -> Option<&str> {
    let cut = path.len().checked_sub(4)?;
    let stem = path.get(..cut).filter(|s| !s.is_empty())?;
    path.get(cut..)?
        .eq_ignore_ascii_case(".mdl")
        .then_some(stem)
}
