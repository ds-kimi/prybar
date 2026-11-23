//! Steam's "too many requests": one request, the wait it asks for, no
//! retry loop, and the older copy kept when there is one.

use std::io::{Read as _, Write as _};
use std::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use prybar_workshop::changelog::{backoff, cached_at};

/// A local server answering every request with 429 and `Retry-After: 7`.
fn busy_server() -> (String, Arc<AtomicUsize>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let url = format!(
        "http://{}/changelog/7",
        listener.local_addr().expect("addr")
    );
    let hits = Arc::new(AtomicUsize::new(0));
    let count = hits.clone();
    std::thread::spawn(move || {
        for mut conn in listener.incoming().flatten() {
            count.fetch_add(1, Ordering::SeqCst);
            let mut buf = [0u8; 2048];
            let _ = conn.read(&mut buf);
            let answer = "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 7\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            let _ = conn.write_all(answer.as_bytes());
        }
    });
    (url, hits)
}

#[test]
fn busy_answer_is_one_request_with_its_wait() {
    let (url, hits) = busy_server();
    let dir = tempfile::tempdir().expect("tempdir");
    let file = dir.path().join("7.html");
    let e = cached_at(&url, &file).expect_err("busy");
    assert!(e.busy());
    assert_eq!(e.retry_after, Some(7));
    assert_eq!(hits.load(Ordering::SeqCst), 1, "no retry loop inside");
    assert!(!e.message.contains("http status"), "{}", e.message);
    let page = "<div class=\"changelog headline\">Update: 1 Jan</div><p id=\"1\">hi</p>";
    std::fs::write(&file, page).expect("old copy");
    let old = std::time::SystemTime::now() - Duration::from_secs(3 * 86_400);
    std::fs::File::options()
        .write(true)
        .open(&file)
        .and_then(|f| f.set_modified(old))
        .expect("age");
    let log = cached_at(&url, &file).expect("stale copy");
    assert!(log.stale && log.age >= 3 * 86_400 - 5);
    assert_eq!(log.changes.len(), 1);
}

#[test]
fn backoff_doubles_honors_retry_after_and_caps() {
    assert_eq!(backoff(0, None), Duration::from_secs(10));
    assert_eq!(backoff(1, None), Duration::from_secs(20));
    assert_eq!(backoff(0, Some(45)), Duration::from_secs(45));
    assert_eq!(backoff(1, Some(3)), Duration::from_secs(20));
    assert_eq!(backoff(40, None), Duration::from_secs(300));
    assert_eq!(backoff(0, Some(u64::MAX)), Duration::from_secs(300));
}
