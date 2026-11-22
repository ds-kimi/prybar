//! Sending the outbox, with the network behind a function pointer so
//! tests never touch it.

use std::time::Duration;

use crate::{Entry, Store};

/// Posts one JSON body to the endpoint.
pub type Transport = fn(&str, &[u8]) -> Result<(), String>;

/// Sends queued events oldest first and logs each result. Stops at the
/// first failure (usually offline) and keeps the rest for next time.
/// Returns how many were sent.
pub fn flush(store: &Store, endpoint: &str, transport: Transport) -> usize {
    if endpoint.is_empty() {
        return 0;
    }
    let mut sent = 0;
    for (path, event) in store.pending() {
        let Ok(body) = serde_json::to_vec(&event) else {
            continue;
        };
        match transport(endpoint, &body) {
            Ok(()) => {
                let _ = std::fs::remove_file(&path);
                let _ = store.append(&Entry::new(endpoint, "sent", event));
                sent += 1;
            }
            Err(why) => {
                let _ = store.append(&Entry::new(endpoint, &format!("failed: {why}"), event));
                break;
            }
        }
    }
    sent
}

/// The real transport: a plain POST, no cookies, short timeout.
pub fn http(endpoint: &str, body: &[u8]) -> Result<(), String> {
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .build()
        .new_agent();
    agent
        .post(endpoint)
        .header("content-type", "application/json")
        .send(body)
        .map(|_| ())
        .map_err(|e| e.to_string())
}
