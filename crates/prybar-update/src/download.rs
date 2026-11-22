use crate::verify::{parse_sums, sha256_file, verify_signature};
use crate::{net, Asset, Config, Release, UpdateError};
use std::path::PathBuf;
use tempfile::TempDir;

/// Largest archive accepted; current releases are a few tens of megabytes.
const MAX_ARCHIVE: u64 = 512 << 20;
const MAX_SMALL: u64 = 1 << 20;

/// An archive whose checksum and signature both matched. The temp dir
/// lives as long as this value.
#[derive(Debug)]
pub struct Verified {
    pub release: Release,
    pub archive: PathBuf,
    pub dir: TempDir,
}

/// Downloads `asset`, then `SHA256SUMS` and `SHA256SUMS.minisig`, and only
/// returns once the signature and the checksum both match.
pub fn download_verified(
    cfg: &Config,
    release: &Release,
    asset: &Asset,
    progress: &mut dyn FnMut(u64, Option<u64>),
) -> Result<Verified, UpdateError> {
    let key = cfg
        .public_key
        .as_deref()
        .ok_or_else(|| UpdateError::Unsigned(release.page.clone()))?;
    let expected = expected_hash(cfg, release, &asset.name, key)?;
    let dir = tempfile::tempdir()?;
    let archive = dir.path().join(safe_name(&asset.name)?);
    let mut file = std::fs::File::create(&archive)?;
    net::fetch_to(cfg, &asset.url, MAX_ARCHIVE, &mut file, progress)?;
    drop(file);
    if sha256_file(&archive)? != expected {
        return Err(UpdateError::BadChecksum(asset.name.clone()));
    }
    Ok(Verified {
        release: release.clone(),
        archive,
        dir,
    })
}

/// Fetches SHA256SUMS and its signature, checks the signature, and returns
/// the hash listed for `name`.
fn expected_hash(
    cfg: &Config,
    release: &Release,
    name: &str,
    key: &str,
) -> Result<[u8; 32], UpdateError> {
    let sums_asset = release
        .asset("SHA256SUMS")
        .ok_or_else(|| UpdateError::MissingAsset("SHA256SUMS".into()))?;
    let sig_asset = release
        .asset("SHA256SUMS.minisig")
        .ok_or_else(|| UpdateError::Unsigned(release.page.clone()))?;
    let sums = small(cfg, sums_asset)?;
    let sig = String::from_utf8(small(cfg, sig_asset)?).map_err(|_| UpdateError::BadSignature)?;
    verify_signature(&sums, &sig, key)?;
    let text = std::str::from_utf8(&sums)
        .map_err(|_| UpdateError::BadRelease("SHA256SUMS not UTF-8".into()))?;
    parse_sums(text)?
        .into_iter()
        .find(|(n, _)| *n == name)
        .map(|(_, h)| h)
        .ok_or_else(|| UpdateError::MissingAsset(format!("{name} in SHA256SUMS")))
}

fn small(cfg: &Config, asset: &Asset) -> Result<Vec<u8>, UpdateError> {
    let mut out = Vec::new();
    net::fetch_to(cfg, &asset.url, MAX_SMALL, &mut out, &mut |_, _| {})?;
    Ok(out)
}

/// Asset names come from the server: keep them to one plain path segment.
fn safe_name(name: &str) -> Result<&str, UpdateError> {
    let ok = !name.is_empty() && !name.contains(['/', '\\', ':']) && name != "." && name != "..";
    ok.then_some(name)
        .ok_or_else(|| UpdateError::BadRelease(format!("asset name {name:.80}")))
}
