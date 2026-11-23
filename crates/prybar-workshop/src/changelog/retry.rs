use std::time::Duration;

/// The first wait when Steam is busy and says nothing about how long.
const FIRST: u64 = 10;
/// Never wait longer than this between two tries.
const CAP: u64 = 300;

/// How long to wait before try number `attempt + 1` (0 is the first
/// retry): doubling from [`FIRST`], never shorter than the server's
/// `Retry-After`, never longer than [`CAP`].
pub fn backoff(attempt: u32, retry_after: Option<u64>) -> Duration {
    let doubled = FIRST.saturating_mul(1u64 << attempt.min(16));
    let wait = doubled.max(retry_after.unwrap_or(0)).clamp(1, CAP);
    Duration::from_secs(wait)
}
