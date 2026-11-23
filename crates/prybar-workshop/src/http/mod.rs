//! Bounded GETs to Steam, polite by construction: a browser-like user
//! agent, a few requests at a time, and "busy" answers kept apart from
//! other failures so callers can back off instead of hammering.

mod gate;

pub use gate::Gate;

use std::io::Read as _;
use std::time::Duration;

/// A failed GET. `status` is set when the server answered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    pub status: Option<u16>,
    /// Seconds from the server's `Retry-After`, when it sent one.
    pub retry_after: Option<u64>,
    pub message: String,
}

impl Failure {
    /// Too many requests or a temporary outage: try later, not now.
    pub fn busy(&self) -> bool {
        matches!(self.status, Some(429 | 503))
    }
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

/// GET `url` with at most `max` bytes of body, waiting for a free slot
/// of `gate` first so a screen full of items never floods Steam.
pub fn get(url: &str, max: u64, gate: &Gate) -> Result<Vec<u8>, Failure> {
    let _pass = gate.enter();
    let fail = |status, retry_after, message: String| Failure {
        status,
        retry_after,
        message,
    };
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(20)))
        .http_status_as_error(false)
        .user_agent(concat!(
            "Mozilla/5.0 (compatible; prybar/",
            env!("CARGO_PKG_VERSION"),
            ")"
        ))
        .build()
        .into();
    let resp = agent
        .get(url)
        .call()
        .map_err(|e| fail(None, None, e.to_string()))?;
    let code = resp.status().as_u16();
    if code != 200 {
        let after = resp.headers().get("retry-after");
        let after = after.and_then(|v| v.to_str().ok()?.trim().parse().ok());
        return Err(fail(Some(code), after, format!("Steam answered {code}")));
    }
    let mut bytes = Vec::new();
    let read = resp
        .into_body()
        .into_reader()
        .take(max + 1)
        .read_to_end(&mut bytes);
    read.map_err(|e| fail(None, None, e.to_string()))?;
    if bytes.len() as u64 > max {
        return Err(fail(
            None,
            None,
            format!("the answer is larger than {} KB", max >> 10),
        ));
    }
    Ok(bytes)
}
