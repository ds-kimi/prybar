mod addon_json;
mod whitelist;
mod wildcard;

pub use addon_json::AddonJson;
pub use whitelist::{is_whitelisted, rejection};

use crate::format::{encode, Header, VERSION};
use crate::{Error, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use wildcard::wildcard;

/// Packs `src` (which must hold an `addon.json`) into `out`; returns the
/// number of files. Fails listing every non-whitelisted file at once.
pub fn pack(src: &Path, out: &Path) -> Result<usize> {
    let json = AddonJson::load(&src.join("addon.json"))?;
    let files = collect(src, &json.ignore)?;
    let bad = scan_files(&files).filter(|f| f.1.is_some()).map(|f| f.0);
    let bad: Vec<String> = bad.collect();
    if !bad.is_empty() {
        return Err(Error::NotWhitelisted(bad));
    }
    let mut contents = Vec::with_capacity(files.len());
    for (rel, path) in files {
        contents.push((rel, std::fs::read(path)?));
    }
    let header = Header {
        version: VERSION,
        name: json.title.clone(),
        description: json.stored_description()?,
        author: "Author Name".into(),
        addon_version: 1,
        ..Header::default()
    };
    std::fs::write(out, encode(&header, &contents))?;
    Ok(contents.len())
}

/// Every file `pack` would take, each with the reason the game refuses
/// it (`None` = allowed), so a UI can list all problems before packing.
pub fn scan(src: &Path) -> Result<Vec<(String, Option<&'static str>)>> {
    let json = AddonJson::load(&src.join("addon.json"))?;
    let files = collect(src, &json.ignore)?;
    Ok(scan_files(&files).collect())
}

fn scan_files(
    files: &[(String, PathBuf)],
) -> impl Iterator<Item = (String, Option<&'static str>)> + '_ {
    files.iter().map(|f| (f.0.clone(), rejection(&f.0)))
}

/// Lowercased like gmad, since the game's filesystem is case-insensitive
/// and the whitelist is written in lowercase. Sorted after lowering so
/// output order does not depend on the source casing.
fn collect(src: &Path, ignore: &[String]) -> Result<Vec<(String, PathBuf)>> {
    let mut out = Vec::new();
    for item in WalkDir::new(src).sort_by_file_name() {
        let item = item?;
        if !item.file_type().is_file() {
            continue;
        }
        let rel = item.path().strip_prefix(src).unwrap_or(item.path());
        let rel = rel.to_str().ok_or_else(|| Error::NonUtf8(rel.into()))?;
        let rel = rel.replace('\\', "/").to_lowercase();
        let ignored = ignore.iter().any(|i| wildcard(&i.to_lowercase(), &rel));
        if rel != "addon.json" && !ignored {
            out.push((rel, item.path().to_path_buf()));
        }
    }
    out.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}
