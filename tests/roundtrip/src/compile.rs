use prybar_tools::compile::{compile_command, parse_log};
use prybar_tools::runner::Runner;
use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};

/// Some shipped collision meshes make studiomdl spin for many minutes;
/// a sample run must finish, so those count as compile failures.
const TIMEOUT: Duration = Duration::from_secs(120);

/// studiomdl with a deadline; `Err` holds the reason for the report.
/// The log goes to a file, not a pipe, so a chatty compile cannot fill
/// the pipe and block before the deadline is checked.
pub fn compile(studiomdl: &Path, game: &Path, qc: &Path) -> Result<(), String> {
    let log_path = game.join("studiomdl.log");
    let log = std::fs::File::create(&log_path).map_err(|e| e.to_string())?;
    let err = log.try_clone().map_err(|e| e.to_string())?;
    let mut child = compile_command(&Runner::for_host(), studiomdl, game, qc)
        .stdin(Stdio::null())
        .stdout(log)
        .stderr(err)
        .spawn()
        .map_err(|e| format!("spawn studiomdl: {e}"))?;
    let start = Instant::now();
    let status = loop {
        if let Some(s) = child.try_wait().map_err(|e| e.to_string())? {
            break s;
        }
        if start.elapsed() > TIMEOUT {
            let _ = child.kill();
            let _ = child.wait();
            return Err(format!("timeout after {}s", TIMEOUT.as_secs()));
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    verdict(
        status.success(),
        &std::fs::read(&log_path).unwrap_or_default(),
    )
}

fn verdict(exited_ok: bool, log: &[u8]) -> Result<(), String> {
    let (errors, _) = parse_log(&String::from_utf8_lossy(log));
    if exited_ok && errors.is_empty() {
        return Ok(());
    }
    let why: Vec<&str> = errors.iter().map(|l| l.text.as_str()).collect();
    Err(if why.is_empty() {
        "studiomdl failed".to_owned()
    } else {
        why.join(" | ")
    })
}
