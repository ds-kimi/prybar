//! Item preview images from Steam's CDN, downloaded once into a cache
//! folder so a list of hundreds of items does not refetch on each run.

use std::hash::{Hash as _, Hasher as _};
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Previews Steam serves are well under this; a bigger body is not one.
const MAX_BYTES: u64 = 4 << 20;

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
    let bytes = download(url)?;
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

fn download(url: &str) -> Result<Vec<u8>, String> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(20)))
        .user_agent(concat!("prybar/", env!("CARGO_PKG_VERSION")))
        .build()
        .into();
    let resp = agent.get(url).call().map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    resp.into_body()
        .into_reader()
        .take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err("preview is larger than 4 MB".into());
    }
    Ok(bytes)
}
