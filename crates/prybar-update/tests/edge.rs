mod common;

use common::fixture::{signed_release, tarball, Faults};
use prybar_update::{apply, download_verified, install_plan, Kind, Plan, UpdateError};

#[test]
fn missing_asset_is_reported() {
    let (cfg, release) = signed_release(tarball("prybar", b"new"), Faults::default());
    assert!(prybar_update::asset_for(&release, "riscv64-none").is_none());
    let mut ghost = release.assets[0].clone();
    ghost.name = "prybar-v0.3.0-ghost.tar.gz".into();
    let got = download_verified(&cfg, &release, &ghost, &mut |_, _| {});
    assert!(matches!(got, Err(UpdateError::MissingAsset(_))));
}

#[test]
fn swaps_an_app_bundle_whole() {
    let dir = tempfile::tempdir().expect("tmp");
    let exe = dir.path().join("Prybar.app/Contents/MacOS/prybar-gui");
    std::fs::create_dir_all(exe.parent().expect("parent")).expect("mkdir");
    std::fs::write(&exe, b"old").expect("write");
    let plan = install_plan(&exe, "p");
    assert_eq!(plan, Plan::Bundle(dir.path().join("Prybar.app")));
    let (cfg, release) = signed_release(
        tarball("Prybar.app/Contents/MacOS/prybar-gui", b"new"),
        Faults::default(),
    );
    let v =
        download_verified(&cfg, &release, &release.assets[0].clone(), &mut |_, _| {}).expect("v");
    apply(&v, Kind::Gui, &plan).expect("apply");
    assert_eq!(std::fs::read(&exe).expect("read"), b"new");
}
