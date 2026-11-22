use std::sync::atomic::{AtomicUsize, Ordering};

use super::{Store, MAX_QUEUED};
use crate::{flush, Event};

fn store() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().expect("temp dir");
    let s = Store::new(dir.path().join("telemetry"));
    (dir, s)
}

#[test]
fn queue_logs_and_drop_queue_empties() {
    let (_d, s) = store();
    s.queue(&Event::app_start("0.1.0")).expect("queued");
    assert_eq!(s.pending().len(), 1);
    assert_eq!(s.entries()[0].status, "queued");
    std::fs::write(s.dir.join("outbox/zz.json"), b"{\"event\":\"x\"}").expect("write");
    assert_eq!(s.pending().len(), 1, "foreign file is dropped");
    s.drop_queue().expect("dropped");
    assert!(s.pending().is_empty());
    s.clear_log().expect("cleared");
    assert!(s.entries().is_empty());
    for _ in 0..MAX_QUEUED + 5 {
        s.queue(&Event::app_start("0.1.0")).expect("queued");
    }
    assert_eq!(s.pending().len(), MAX_QUEUED);
}

static CALLS: AtomicUsize = AtomicUsize::new(0);

#[test]
fn flush_sends_logs_and_keeps_on_failure() {
    let (_d, s) = store();
    s.queue(&Event::app_start("0.1.0")).expect("queued");
    s.queue(&Event::app_start("0.1.0")).expect("queued");
    assert_eq!(flush(&s, "", |_, _| Ok(())), 0, "no endpoint, no send");
    assert_eq!(
        flush(&s, "https://c.invalid/e", |_, _| Err("offline".into())),
        0
    );
    assert_eq!(s.pending().len(), 2);
    let ok = |_: &str, _: &[u8]| {
        CALLS.fetch_add(1, Ordering::SeqCst);
        Ok(())
    };
    assert_eq!(flush(&s, "https://c.invalid/e", ok), 2);
    assert!(s.pending().is_empty());
    let last = s.entries().pop().expect("logged");
    assert_eq!(
        (last.status.as_str(), last.endpoint.as_str()),
        ("sent", "https://c.invalid/e")
    );
}
