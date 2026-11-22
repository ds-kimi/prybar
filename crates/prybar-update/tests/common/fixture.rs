use super::{release_json, serve, sign, Route};
use prybar_update::{Config, Release};
use sha2::{Digest, Sha256};

/// Knobs to break one part of an otherwise valid signed release.
#[derive(Default)]
pub struct Faults {
    pub wrong_hash: bool,
    pub wrong_key: bool,
    pub no_signature: bool,
}

/// A gzipped tarball holding `prybar-v0.3.0-<target>/prybar` with `body`.
pub fn tarball(name: &str, body: &[u8]) -> Vec<u8> {
    let gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    let mut tar = tar::Builder::new(gz);
    let mut head = tar::Header::new_gnu();
    head.set_size(body.len() as u64);
    head.set_mode(0o755);
    head.set_cksum();
    tar.append_data(&mut head, format!("prybar-v0.3.0-x/{name}"), body)
        .expect("append");
    tar.into_inner().expect("tar").finish().expect("gz")
}

/// The SHA256SUMS text for one archive, optionally with a wrong hash.
fn sums_for(name: &str, archive: &[u8], wrong: bool) -> Vec<u8> {
    let hash = if wrong {
        [0u8; 32].to_vec()
    } else {
        Sha256::digest(archive).to_vec()
    };
    format!("{}  {name}\n", hex::encode(hash)).into_bytes()
}

/// Serves a v0.3.0 release with one archive, SHA256SUMS and its signature.
pub fn signed_release(archive: Vec<u8>, faults: Faults) -> (Config, Release) {
    let name = "prybar-v0.3.0-x.tar.gz";
    let sums = sums_for(name, &archive, faults.wrong_hash);
    let (mut key, sig) = sign(&sums);
    if faults.wrong_key {
        key = sign(b"other").0;
    }
    let mut names = vec![name, "SHA256SUMS"];
    if !faults.no_signature {
        names.push("SHA256SUMS.minisig");
    }
    let route = |p: &str, b: Vec<u8>| Route {
        path: format!("/{p}"),
        status: 200,
        etag: None,
        body: b,
    };
    let base = serve(vec![
        route(name, archive),
        route("SHA256SUMS", sums),
        route("SHA256SUMS.minisig", sig.into_bytes()),
    ]);
    let mut cfg = Config::new("0.2.0");
    cfg.public_key = Some(key);
    cfg.cache_dir = None;
    (
        cfg,
        Release::parse(&release_json(&base, "v0.3.0", &names)).expect("release"),
    )
}

/// A local port with nothing listening: what "offline" looks like.
pub fn dead_server() -> String {
    let closed = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    format!("http://{}", closed.local_addr().expect("addr"))
}
