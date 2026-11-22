//! Local HTTP server and throwaway signing key, so tests never touch the
//! real network or the real release key.
#![allow(dead_code)]

pub mod fixture;

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

/// One served path; `etag` makes the server answer 304 to a matching
/// `If-None-Match`.
#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub status: u16,
    pub etag: Option<String>,
    pub body: Vec<u8>,
}

/// Serves `routes` on a free local port until the test process exits.
pub fn serve(routes: Vec<Route>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let base = format!("http://{}", listener.local_addr().expect("addr"));
    let routes = Arc::new(routes);
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let routes = Arc::clone(&routes);
            std::thread::spawn(move || handle(stream, &routes));
        }
    });
    base
}

fn handle(mut stream: TcpStream, routes: &[Route]) {
    let mut reader = BufReader::new(stream.try_clone().expect("clone"));
    let (mut path, mut inm, mut line) = (String::new(), None, String::new());
    while reader.read_line(&mut line).is_ok_and(|n| n > 2) {
        if let Some(rest) = line.strip_prefix("GET ") {
            path = rest.split(' ').next().unwrap_or("").to_string();
        }
        if let Some(v) = line.to_ascii_lowercase().strip_prefix("if-none-match:") {
            inm = Some(v.trim().to_string());
        }
        line.clear();
    }
    let route = routes.iter().find(|r| r.path == path);
    let (status, body, etag) = match route {
        Some(r) if r.etag.is_some() && r.etag.as_ref().map(|e| e.to_ascii_lowercase()) == inm => {
            (304, Vec::new(), None)
        }
        Some(r) => (r.status, r.body.clone(), r.etag.clone()),
        None => (404, Vec::new(), None),
    };
    let tag = etag.map(|e| format!("ETag: {e}\r\n")).unwrap_or_default();
    let head = format!(
        "HTTP/1.1 {status} X\r\nContent-Length: {}\r\n{tag}Connection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(&body);
}

/// Generates a fresh key pair and signs `data`: (public key file, signature file).
pub fn sign(data: &[u8]) -> (String, String) {
    let kp = minisign::KeyPair::generate_unencrypted_keypair().expect("keypair");
    let sig = minisign::sign(Some(&kp.pk), &kp.sk, data, None, None).expect("sign");
    (kp.pk.to_box().expect("box").to_string(), sig.to_string())
}

/// A GitHub-style release JSON whose assets live under `base`.
pub fn release_json(base: &str, tag: &str, names: &[&str]) -> Vec<u8> {
    let assets: Vec<serde_json::Value> = names
        .iter()
        .map(|n| serde_json::json!({"name": n, "browser_download_url": format!("{base}/{n}"), "size": 1}))
        .collect();
    let v = serde_json::json!({"tag_name": tag, "html_url": format!("{base}/page"), "draft": false, "prerelease": false, "assets": assets});
    serde_json::to_vec(&v).expect("json")
}
