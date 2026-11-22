use super::mdl_common::{build, geometry};
use assert_cmd::Command;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// A `prybar` command with a private config file, so tests never read
/// or write the user's real game config.
pub fn prybar(tmp: &Path) -> Command {
    let mut cmd = Command::cargo_bin("prybar").unwrap();
    cmd.env("PRYBAR_CONFIG", tmp.join("config.toml"));
    cmd
}

/// `<dir>/models/<name>.mdl` with its VVD and VTX; returns the .mdl path.
pub fn model(dir: &Path, name: &str) -> PathBuf {
    let models = dir.join("models");
    std::fs::create_dir_all(&models).unwrap();
    std::fs::write(models.join(format!("{name}.vvd")), geometry::vvd()).unwrap();
    std::fs::write(models.join(format!("{name}.dx90.vtx")), geometry::vtx()).unwrap();
    let mdl = models.join(format!("{name}.mdl"));
    std::fs::write(&mdl, build()).unwrap();
    mdl
}

/// Runs with `--json`, checks the exit code and parses stdout.
pub fn json(mut cmd: Command, code: i32) -> Value {
    let out = cmd
        .arg("--json")
        .assert()
        .code(code)
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&out)
        .unwrap_or_else(|e| panic!("{e}: {}", String::from_utf8_lossy(&out)))
}

/// A folder holding one model, packed as `<tmp>/pak_dir.vpk`.
pub fn vpk(tmp: &Path) -> PathBuf {
    let src = tmp.join("src");
    model(&src, "crate");
    let out = tmp.join("pak_dir.vpk");
    let opts = prybar_vpk::PackOptions {
        version: 2,
        chunk_size: None,
    };
    prybar_vpk::pack(&src, &out, &opts).unwrap();
    out
}
