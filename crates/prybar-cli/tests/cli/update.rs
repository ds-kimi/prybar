use super::fixture::{json, prybar};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use tempfile::tempdir;

/// Serves one GitHub-style release JSON for `tag` on a local port.
fn release_server(tag: &str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let target = prybar_update::target_triple().unwrap_or("none");
    let names = [format!("prybar-{tag}-{target}.tar.gz"), "SHA256SUMS".into()];
    let assets: Vec<_> = names
        .iter()
        .map(|n| serde_json::json!({"name": n, "browser_download_url": format!("{base}/{n}")}))
        .collect();
    let body =
        serde_json::json!({"tag_name": tag, "html_url": format!("{base}/page"), "assets": assets})
            .to_string();
    std::thread::spawn(move || {
        for mut s in listener.incoming().flatten() {
            let mut r = BufReader::new(s.try_clone().unwrap());
            let mut line = String::new();
            while r.read_line(&mut line).is_ok_and(|n| n > 2) {
                line.clear();
            }
            let head = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = s.write_all(head.as_bytes());
            let _ = s.write_all(body.as_bytes());
        }
    });
    base
}

#[test]
fn version_check_reports_newer_release_and_offline() {
    let tmp = tempdir().unwrap();
    let mut cmd = prybar(tmp.path());
    cmd.env("PRYBAR_UPDATE_URL", release_server("v99.0.0"));
    cmd.args(["version", "--check"]);
    let v = json(cmd, 0);
    assert_eq!(v["latest"], "99.0.0");
    assert_eq!(v["update_available"], true);
    assert_eq!(v["current"], env!("CARGO_PKG_VERSION"));
    let mut cmd = prybar(tmp.path());
    cmd.env("PRYBAR_UPDATE_CACHE", tmp.path().join("other"));
    cmd.args(["version", "--check"]);
    let v = json(cmd, 0);
    assert!(v["latest"].is_null());
    assert_eq!(v["update_available"], false);
}

#[test]
fn self_update_up_to_date_and_unsigned_fall_back_to_a_link() {
    let tmp = tempdir().unwrap();
    let mut cmd = prybar(tmp.path());
    let current = format!("v{}", env!("CARGO_PKG_VERSION"));
    cmd.env("PRYBAR_UPDATE_URL", release_server(&current));
    cmd.arg("self-update");
    assert_eq!(json(cmd, 0)["action"], "up_to_date");
    // No SHA256SUMS.minisig: nothing may be installed, only a link shown.
    let mut cmd = prybar(tmp.path());
    cmd.env("PRYBAR_UPDATE_URL", release_server("v99.0.0"));
    cmd.env("PRYBAR_UPDATE_CACHE", tmp.path().join("other"));
    cmd.arg("self-update");
    let v = json(cmd, 0);
    assert_eq!(v["action"], "manual");
    assert!(v["detail"].as_str().unwrap().contains("/page"));
}

#[test]
fn notice_never_touches_json_stdout() {
    let tmp = tempdir().unwrap();
    let mut cmd = prybar(tmp.path());
    cmd.env_remove("PRYBAR_NO_UPDATE_CHECK");
    cmd.env("PRYBAR_UPDATE_URL", release_server("v99.0.0"));
    cmd.args(["games"]);
    let v = json(cmd, 0);
    assert!(v["games"].is_array());
}
