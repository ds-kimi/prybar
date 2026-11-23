//! Item preview images from Steam's CDN, downloaded once into a cache
//! folder so a list of hundreds of items does not refetch on each run.

use std::hash::{Hash as _, Hasher as _};
use std::path::{Path, PathBuf};

/// Previews Steam serves are well under this; a bigger body is not one.
const MAX_BYTES: u64 = 4 << 20;
/// The CDN copes with a few at once; a screen of cards must not open
/// twenty connections.
static GATE: crate::http::Gate = crate::http::Gate::new(4);

/// The cached file for `url`, downloading it first when missing. Only
/// https is fetched: the URL comes from Steam but is still outside input.
pub fn cached(url: &str, dir: &Path) -> Result<PathBuf, String> {
    let path = dir.join(name(url));
    if path.is_file() {
        return Ok(path);
    }
    if !url.starts_with("https://") {
        return Err("preview is not an https link".into());
    }
    let bytes = crate::http::get(url, MAX_BYTES, &GATE).map_err(|e| e.message)?;
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    // Write aside then rename, so a crash never leaves half an image
    // that the next run would take for a cached one.
    let part = path.with_extension("part");
    std::fs::write(&part, bytes).map_err(|e| e.to_string())?;
    std::fs::rename(&part, &path).map_err(|e| e.to_string())?;
    Ok(path)
}

/// A stable file name per URL; the extension does not matter to the
/// image decoder, which reads the header.
fn name(url: &str) -> String {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut h);
    format!("{:016x}.img", h.finish())
}
