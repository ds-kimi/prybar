use prybar_service::package::Package;
use prybar_service::source::matches;

/// Model stems of `pkg` that ship a `.vvd`, every `stride`-th one so a
/// small sample still spans the whole alphabet of folders.
pub fn sample(pkg: &Package, want: usize) -> Vec<String> {
    let entries = pkg.entries();
    let lower: std::collections::HashSet<String> = entries
        .iter()
        .map(|e| e.path.to_ascii_lowercase())
        .collect();
    let mut stems: Vec<String> = entries
        .iter()
        .filter(|e| matches("*.mdl", &e.path))
        .filter_map(|e| e.path.get(..e.path.len().checked_sub(4)?))
        .filter(|s| lower.contains(&format!("{}.vvd", s.to_ascii_lowercase())))
        .map(str::to_owned)
        .collect();
    stems.sort();
    let stride = (stems.len() / want.max(1)).max(1);
    stems.into_iter().step_by(stride).take(want).collect()
}
