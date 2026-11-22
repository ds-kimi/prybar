use std::path::{Path, PathBuf};

/// Archive entry names are attacker-controlled, so only plain relative
/// components may reach the filesystem. `:` is refused because Windows
/// reads it as a drive prefix or an alternate data stream.
pub fn safe_join(root: &Path, rel: &str) -> Option<PathBuf> {
    let mut out = root.to_path_buf();
    let mut any = false;
    for part in rel.split(['/', '\\']) {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." || part.contains([':', '\0']) {
            return None;
        }
        out.push(part);
        any = true;
    }
    any.then_some(out)
}
