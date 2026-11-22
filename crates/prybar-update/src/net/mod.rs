mod cache;

pub use cache::{load, store};

use crate::{Config, UpdateError};
use std::io::Read;

/// Largest API answer accepted; a release JSON is a few kilobytes.
const MAX_JSON: u64 = 1 << 20;

/// A blocking client with one global timeout, so a stalled network can
/// never hang the caller.
pub fn agent(cfg: &Config) -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(cfg.timeout))
        .http_status_as_error(false)
        .user_agent(concat!("prybar-update/", env!("CARGO_PKG_VERSION")))
        .build()
        .into()
}

/// A fresh answer: body and its ETag.
pub type Fresh = (Vec<u8>, Option<String>);

/// GETs the release JSON, sending the cached ETag. Returns `None` on 304.
pub fn fetch_json(cfg: &Config, etag: Option<&str>) -> Result<Option<Fresh>, UpdateError> {
    let mut req = agent(cfg)
        .get(&cfg.api_url)
        .header("Accept", "application/vnd.github+json");
    if let Some(tag) = etag {
        req = req.header("If-None-Match", tag);
    }
    let mut resp = req
        .call()
        .map_err(|e| UpdateError::Network(e.to_string()))?;
    let status = resp.status().as_u16();
    if status == 304 {
        return Ok(None);
    }
    if status != 200 {
        return Err(UpdateError::Network(format!("HTTP {status}")));
    }
    let new_tag = resp
        .headers()
        .get("etag")
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    let body = resp
        .body_mut()
        .with_config()
        .limit(MAX_JSON)
        .read_to_vec()
        .map_err(|e| UpdateError::Network(e.to_string()))?;
    Ok(Some((body, new_tag)))
}

/// Streams `url` into `out`, calling `progress(done, total)`; refuses more
/// than `max` bytes so a hostile server cannot fill the disk.
pub fn fetch_to(
    cfg: &Config,
    url: &str,
    max: u64,
    out: &mut dyn std::io::Write,
    progress: &mut dyn FnMut(u64, Option<u64>),
) -> Result<u64, UpdateError> {
    let mut resp = agent(cfg)
        .get(url)
        .call()
        .map_err(|e| UpdateError::Network(e.to_string()))?;
    let status = resp.status().as_u16();
    if status != 200 {
        return Err(UpdateError::Network(format!("HTTP {status} for {url}")));
    }
    let total = resp.body().content_length();
    let mut reader = resp.body_mut().as_reader().take(max + 1);
    copy_capped(&mut reader, max, out, &mut |done| progress(done, total))
}

fn copy_capped(
    reader: &mut dyn Read,
    max: u64,
    out: &mut dyn std::io::Write,
    progress: &mut dyn FnMut(u64),
) -> Result<u64, UpdateError> {
    let (mut done, mut buf) = (0u64, vec![0u8; 64 * 1024]);
    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| UpdateError::Network(e.to_string()))?;
        if n == 0 {
            return Ok(done);
        }
        done += n as u64;
        if done > max {
            return Err(UpdateError::TooLarge(max));
        }
        out.write_all(&buf[..n])?;
        progress(done);
    }
}
