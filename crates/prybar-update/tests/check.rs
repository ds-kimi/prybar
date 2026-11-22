mod common;

use common::fixture::dead_server;
use common::{release_json, serve, Route};
use prybar_update::{check, Config, Status};
use std::time::Duration;

fn cfg(base: &str, current: &str, cache: Option<&std::path::Path>) -> Config {
    let mut c = Config::new(current);
    c.api_url = format!("{base}/latest");
    c.cache_dir = cache.map(|p| p.to_path_buf());
    c.timeout = Duration::from_secs(3);
    c
}

#[test]
fn newer_older_same() {
    let base = serve(vec![Route {
        path: "/latest".into(),
        status: 200,
        etag: None,
        body: release_json("http://x", "v0.3.0", &[]),
    }]);
    assert!(
        matches!(check(&cfg(&base, "0.2.1", None)), Status::Available(r) if r.version.to_string() == "0.3.0")
    );
    assert_eq!(check(&cfg(&base, "0.3.0", None)), Status::UpToDate);
    assert_eq!(check(&cfg(&base, "1.0.0", None)), Status::UpToDate);
}

#[test]
fn offline_and_bad_answers_are_unknown() {
    assert!(matches!(
        check(&cfg(&dead_server(), "0.1.0", None)),
        Status::Unknown(_)
    ));
    let base = serve(vec![
        Route {
            path: "/latest".into(),
            status: 500,
            etag: None,
            body: vec![],
        },
        Route {
            path: "/pre/latest".into(),
            status: 200,
            etag: None,
            body: release_json("x", "v9.0.0-rc1", &[]),
        },
    ]);
    assert!(matches!(
        check(&cfg(&base, "0.1.0", None)),
        Status::Unknown(_)
    ));
    assert!(matches!(
        check(&cfg(&format!("{base}/pre"), "0.1.0", None)),
        Status::Unknown(_)
    ));
}

#[test]
fn etag_cache_answers_304() {
    let dir = tempfile::tempdir().expect("tmp");
    let body = release_json("http://x", "v0.5.0", &[]);
    let base = serve(vec![Route {
        path: "/latest".into(),
        status: 200,
        etag: Some("\"abc\"".into()),
        body,
    }]);
    let c = cfg(&base, "0.1.0", Some(dir.path()));
    assert!(matches!(check(&c), Status::Available(_)));
    // Second call sends the ETag, gets 304 and still knows the release.
    assert!(matches!(check(&c), Status::Available(_)));
    assert!(matches!(prybar_update::cached_status(&c), Some((Status::Available(_), t)) if t > 0));
}
