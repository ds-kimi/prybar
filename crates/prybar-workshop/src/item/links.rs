use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Item id to the local addon folder it is built from, so "Update
/// content" needs no picking. Stored as `id<TAB>path` lines.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Links(pub BTreeMap<u64, PathBuf>);

impl Links {
    /// A missing or damaged file is an empty map: links are a
    /// convenience, never a reason to fail.
    pub fn load(file: &Path) -> Self {
        let text = std::fs::read_to_string(file).unwrap_or_default();
        let rows = text.lines().filter_map(|l| {
            let (id, path) = l.split_once('\t')?;
            Some((id.trim().parse().ok()?, PathBuf::from(path)))
        });
        Links(
            rows.filter(|(_, p): &(u64, PathBuf)| !p.as_os_str().is_empty())
                .collect(),
        )
    }

    /// Paths with a newline cannot be stored in this format and are
    /// skipped rather than corrupting the file.
    pub fn save(&self, file: &Path) -> std::io::Result<()> {
        let mut out = String::new();
        for (id, path) in &self.0 {
            let p = path.to_string_lossy();
            if !p.contains(['\n', '\r']) {
                out.push_str(&format!("{id}\t{p}\n"));
            }
        }
        if let Some(dir) = file.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let part = file.with_extension("part");
        std::fs::write(&part, out)?;
        std::fs::rename(part, file)
    }
}
