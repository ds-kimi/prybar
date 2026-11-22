use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use steamworks::Client;

use crate::WorkshopError;

/// Big addons on slow links take a while; past this Steam is stuck.
const TIMEOUT: Duration = Duration::from_secs(3 * 3600);
const TICK: Duration = Duration::from_millis(50);

/// Steam answers through callbacks that only fire while we pump them,
/// so block here pumping until the result arrives.
pub fn wait<T>(
    client: &Client,
    rx: &Receiver<T>,
    mut tick: impl FnMut(),
) -> Result<T, WorkshopError> {
    let start = Instant::now();
    loop {
        client.run_callbacks();
        if let Ok(value) = rx.try_recv() {
            return Ok(value);
        }
        if start.elapsed() > TIMEOUT {
            return Err(WorkshopError::Steam("timed out waiting for Steam".into()));
        }
        tick();
        std::thread::sleep(TICK);
    }
}

pub fn init(app_id: u32) -> Result<Client, WorkshopError> {
    Client::init_app(steamworks::AppId(app_id))
        .map_err(|e| WorkshopError::Steam(format!("init failed (is Steam running?): {e}")))
}
