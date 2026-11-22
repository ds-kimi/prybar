use std::path::Path;
use std::process::Stdio;

use super::{compile_command, parse_log, CompileReport};
use crate::runner::Runner;
use crate::ToolError;

/// A runaway compile can print megabytes; agents need the head.
const MAX_LOG: usize = 4 << 20;

/// Runs studiomdl to completion and reports what it said.
pub fn compile(
    runner: &Runner,
    studiomdl: &Path,
    game_dir: &Path,
    qc: &Path,
) -> Result<CompileReport, ToolError> {
    let output = compile_command(runner, studiomdl, game_dir, qc)
        .stdin(Stdio::null())
        .output()
        .map_err(|source| ToolError::Spawn {
            exe: studiomdl.to_path_buf(),
            source,
        })?;
    let mut log = String::from_utf8_lossy(&output.stdout).into_owned();
    log.push_str(&String::from_utf8_lossy(&output.stderr));
    // Parse before truncating so a late ERROR still fails the report.
    let (errors, warnings) = parse_log(&log);
    truncate(&mut log);
    Ok(CompileReport {
        success: output.status.success() && errors.is_empty(),
        exit_code: output.status.code(),
        errors,
        warnings,
        log,
    })
}

fn truncate(log: &mut String) {
    if log.len() > MAX_LOG {
        let mut end = MAX_LOG;
        while !log.is_char_boundary(end) {
            end -= 1;
        }
        log.truncate(end);
    }
}
