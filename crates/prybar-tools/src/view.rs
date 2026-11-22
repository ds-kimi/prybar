//! hlmv wrapper: open a model in the viewer without waiting for it.

use std::path::Path;
use std::process::Stdio;

use crate::runner::Runner;
use crate::ToolError;

/// Starts hlmv detached and returns its PID. The child is not waited on:
/// the viewer lives as long as the user keeps it open, past our process.
pub fn view(runner: &Runner, hlmv: &Path, game_dir: &Path, mdl: &Path) -> Result<u32, ToolError> {
    let child = runner
        .command(hlmv)
        .arg("-game")
        .arg(game_dir)
        .arg(mdl)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|source| ToolError::Spawn {
            exe: hlmv.to_path_buf(),
            source,
        })?;
    Ok(child.id())
}
