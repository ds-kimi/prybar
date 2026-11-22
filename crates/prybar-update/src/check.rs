use crate::net;
use crate::{Config, Release};

/// Result of an update check. `Unknown` covers offline, rate limits and bad
/// answers: the caller just says nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    UpToDate,
    Available(Release),
    Unknown(String),
}

/// Asks GitHub for the latest release, using the ETag cache when present.
pub fn check(cfg: &Config) -> Status {
    let dir = cfg.cache_dir.as_deref();
    let cached = net::load(dir);
    let etag = cached.as_ref().and_then(|c| c.etag.as_deref());
    let body = match net::fetch_json(cfg, etag) {
        Ok(Some((body, tag))) => {
            net::store(dir, tag, &body);
            body
        }
        Ok(None) => match cached {
            Some(c) => {
                net::store(dir, c.etag.clone(), c.body.as_bytes());
                c.body.into_bytes()
            }
            None => return Status::Unknown("304 without cache".into()),
        },
        Err(e) => return Status::Unknown(e.to_string()),
    };
    status_of(cfg, &body)
}

/// The status from the last cached answer, without any network, plus the
/// time it was fetched. Lets the CLI rate limit to one check a day.
pub fn cached_status(cfg: &Config) -> Option<(Status, u64)> {
    let c = net::load(cfg.cache_dir.as_deref())?;
    Some((status_of(cfg, c.body.as_bytes()), c.checked_at))
}

fn status_of(cfg: &Config, body: &[u8]) -> Status {
    match Release::parse(body) {
        Ok(r) if r.version > cfg.current => Status::Available(r),
        Ok(_) => Status::UpToDate,
        Err(e) => Status::Unknown(e.to_string()),
    }
}
