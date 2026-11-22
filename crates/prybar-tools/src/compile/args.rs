use std::path::Path;
use std::process::Command;

use crate::runner::Runner;

/// `-nop4` because no modder has Perforce; `-verbose` because the log is
/// the only diagnostic studiomdl gives and we parse it.
pub fn compile_command(runner: &Runner, studiomdl: &Path, game_dir: &Path, qc: &Path) -> Command {
    // Absolute because the child runs inside the QC folder (see below).
    let abs = |p: &Path| std::path::absolute(p).unwrap_or_else(|_| p.to_path_buf());
    let (studiomdl, game_dir, qc) = (abs(studiomdl), abs(game_dir), abs(qc));
    let mut cmd = runner.command(&studiomdl);
    cmd.arg("-game")
        .arg(&game_dir)
        .arg("-nop4")
        .arg("-verbose")
        .arg(&qc);
    if let Some(dir) = qc.parent().filter(|d| !d.as_os_str().is_empty()) {
        // $cd/$include paths in a QC are relative to the QC itself.
        cmd.current_dir(dir);
    }
    cmd
}
