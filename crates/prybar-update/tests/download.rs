mod common;

use common::fixture::{signed_release, tarball, Faults};
use prybar_update::{download_verified, UpdateError};

fn run(faults: Faults) -> Result<prybar_update::Verified, UpdateError> {
    let (cfg, release) = signed_release(tarball("prybar", b"new"), faults);
    let asset = release.assets[0].clone();
    download_verified(&cfg, &release, &asset, &mut |_, _| {})
}

#[test]
fn good_release_verifies() {
    let v = run(Faults::default()).expect("verified");
    assert!(v.archive.exists());
    assert_eq!(v.release.version.to_string(), "0.3.0");
}

#[test]
fn bad_checksum_signature_or_unsigned_are_refused() {
    let wrong = Faults {
        wrong_hash: true,
        ..Faults::default()
    };
    assert!(matches!(run(wrong), Err(UpdateError::BadChecksum(_))));
    let key = Faults {
        wrong_key: true,
        ..Faults::default()
    };
    assert!(matches!(run(key), Err(UpdateError::BadSignature)));
    let unsigned = Faults {
        no_signature: true,
        ..Faults::default()
    };
    assert!(matches!(run(unsigned), Err(UpdateError::Unsigned(_))));
}

#[test]
fn no_embedded_key_means_link_only() {
    let (mut cfg, release) = signed_release(tarball("prybar", b"new"), Faults::default());
    cfg.public_key = None;
    let asset = release.assets[0].clone();
    let got = download_verified(&cfg, &release, &asset, &mut |_, _| {});
    assert!(matches!(got, Err(UpdateError::Unsigned(page)) if page.ends_with("/page")));
}
