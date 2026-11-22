//! Panic hook that leaves a crash report in the outbox.

use std::panic::PanicHookInfo;

use super::sanitize;
use crate::{Event, Store};

/// Chains onto the existing hook. The report is only written locally
/// here; whether it is ever sent is decided on the next launch.
pub fn install(store: Store, version: &'static str) {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = store.queue(&report(info, version));
        previous(info);
    }));
}

fn report(info: &PanicHookInfo<'_>, version: &str) -> Event {
    let payload = info.payload();
    let raw = payload
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "panic".into());
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .ok();
    let trace = std::backtrace::Backtrace::force_capture().to_string();
    Event::crash(
        version,
        sanitize::message(&raw, user.as_deref()),
        sanitize::frames(&trace),
    )
}
