mod common;

use common::fixture::{signed_release, tarball, Faults};
use prybar_update::{apply, download_verified, install_plan, Applied, Kind, Plan};

fn verified() -> prybar_update::Verified {
    let (cfg, release) = signed_release(tarball(Kind::Cli.file_name(), b"new"), Faults::default());
    let asset = release.assets[0].clone();
    download_verified(&cfg, &release, &asset, &mut |_, _| {}).expect("verified")
}

#[test]
fn replaces_a_binary_in_a_writable_dir() {
    let dir = tempfile::tempdir().expect("tmp");
    let exe = dir.path().join(Kind::Cli.file_name());
    std::fs::write(&exe, b"old").expect("write");
    let plan = install_plan(&exe, "https://example.invalid/page");
    assert_eq!(plan, Plan::Binary(exe.clone()));
    let done = apply(&verified(), Kind::Cli, &plan).expect("apply");
    assert_eq!(done, Applied::Installed("0.3.0".into()));
    assert_eq!(std::fs::read(&exe).expect("read"), b"new");
}

#[test]
fn managed_installs_get_a_command() {
    let plan = install_plan(
        std::path::Path::new("/opt/homebrew/Cellar/prybar/0.2/bin/prybar"),
        "p",
    );
    assert_eq!(plan, Plan::Manual("brew upgrade prybar".into()));
    let cargo = install_plan(std::path::Path::new("/home/u/.cargo/bin/prybar"), "p");
    assert!(matches!(&cargo, Plan::Manual(c) if c.starts_with("cargo install")));
    let applied = apply(&verified(), Kind::Cli, &cargo).expect("apply");
    assert!(matches!(applied, Applied::Manual(_)));
}

#[cfg(unix)]
#[test]
fn read_only_dir_gets_the_link() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().expect("tmp");
    let exe = dir.path().join("prybar");
    std::fs::write(&exe, b"old").expect("write");
    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o555)).expect("chmod");
    let plan = install_plan(&exe, "https://example.invalid/page");
    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o755)).expect("chmod");
    assert_eq!(
        plan,
        Plan::Manual("download the update from https://example.invalid/page".into())
    );
}
