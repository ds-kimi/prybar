//! studiomdl with its output handed over line by line, so a window can
//! show the log while the compile runs, and stop it on request.

use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Child, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::time::Duration;

use super::{compile_command, parse_log, CompileReport, LogLine};
use crate::runner::Runner;
use crate::ToolError;

/// Same cap as the buffered compile: the log kept for the report.
const MAX_LOG: usize = 4 << 20;
/// A line with no newline in sight is cut here so one never fills memory.
const MAX_LINE: u64 = 64 << 10;
/// How often a quiet compile looks at the cancel flag.
const POLL: Duration = Duration::from_millis(50);

/// Runs studiomdl, calling `on_line` for each output line (stdout and
/// stderr, in arrival order). Setting `cancel` kills the compile; the
/// report then says what was printed until then.
pub fn compile_streamed(
    runner: &Runner,
    (studiomdl, game_dir, qc): (&Path, &Path, &Path),
    on_line: &mut dyn FnMut(&str),
    cancel: &AtomicBool,
) -> Result<CompileReport, ToolError> {
    let mut child = compile_command(runner, studiomdl, game_dir, qc)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| ToolError::Spawn {
            exe: studiomdl.to_path_buf(),
            source,
        })?;
    let (tx, rx) = mpsc::channel();
    if let Some(out) = child.stdout.take() {
        pipe(out, tx.clone());
    }
    if let Some(err) = child.stderr.take() {
        pipe(err, tx);
    }
    pump(&mut child, &rx, on_line, cancel)
}

/// Reads one pipe on its own thread so neither stream blocks the other.
fn pipe(src: impl Read + Send + 'static, tx: Sender<String>) {
    std::thread::spawn(move || {
        let mut reader = BufReader::new(src);
        let mut buf = Vec::new();
        loop {
            buf.clear();
            match (&mut reader).take(MAX_LINE).read_until(b'\n', &mut buf) {
                Ok(0) | Err(_) => break,
                Ok(_) => {}
            }
            let line = String::from_utf8_lossy(&buf);
            if tx
                .send(line.trim_end_matches(['\r', '\n']).to_owned())
                .is_err()
            {
                break;
            }
        }
    });
}

/// Errors and warnings are taken per line as they arrive, so a late
/// ERROR still fails the report even past the kept log.
fn pump(
    child: &mut Child,
    rx: &mpsc::Receiver<String>,
    on_line: &mut dyn FnMut(&str),
    cancel: &AtomicBool,
) -> Result<CompileReport, ToolError> {
    let (mut log, mut errors, mut warnings, mut n) = (String::new(), vec![], vec![], 0);
    loop {
        if cancel.load(Ordering::Relaxed) {
            let _ = child.kill();
        }
        let line = match rx.recv_timeout(POLL) {
            Ok(line) => line,
            Err(RecvTimeoutError::Timeout) => continue,
            Err(RecvTimeoutError::Disconnected) => break,
        };
        n += 1;
        on_line(&line);
        let (e, w) = parse_log(&line);
        let at = |v: Vec<LogLine>| v.into_iter().map(move |l| LogLine { line: n, ..l });
        errors.extend(at(e));
        warnings.extend(at(w));
        if log.len() + line.len() < MAX_LOG {
            log.push_str(&line);
            log.push('\n');
        }
    }
    finish(child, log, errors, warnings)
}

fn finish(
    child: &mut Child,
    log: String,
    errors: Vec<LogLine>,
    warnings: Vec<LogLine>,
) -> Result<CompileReport, ToolError> {
    let status = child.wait().map_err(|source| ToolError::Io {
        path: "studiomdl".into(),
        source,
    })?;
    Ok(CompileReport {
        success: status.success() && errors.is_empty(),
        exit_code: status.code(),
        errors,
        warnings,
        log,
    })
}
