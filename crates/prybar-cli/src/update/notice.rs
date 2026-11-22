use crate::cli::Command;
use crate::output::Output;
use prybar_update::{check, checks_disabled, fresh_status, Config, Status, CHECK_EVERY};
use std::io::IsTerminal;
use std::thread::JoinHandle;

/// The update notice for one run. A due check runs on a background thread
/// so the command never waits on the network; if it has not finished when
/// the command ends, its answer is cached and shown on the next run.
pub enum Notice {
    Off,
    Ready(Status),
    Pending(JoinHandle<Status>),
}

impl Notice {
    pub fn start(cmd: &Command, out: &Output) -> Self {
        let own = matches!(cmd, Command::Version(_) | Command::SelfUpdate);
        if own || out.quiet || checks_disabled() || !std::io::stderr().is_terminal() {
            return Self::Off;
        }
        let cfg = Config::from_env(env!("CARGO_PKG_VERSION"));
        match fresh_status(&cfg, CHECK_EVERY) {
            Some(status) => Self::Ready(status),
            None => Self::Pending(std::thread::spawn(move || check(&cfg))),
        }
    }

    /// Prints the notice on stderr only: stdout may carry `--json`. A
    /// pending check gets a short grace so fast commands still fill the
    /// cache on a normal network, without a noticeable wait offline.
    pub fn finish(self) {
        if let Self::Pending(h) = &self {
            for _ in 0..30 {
                if h.is_finished() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
        let status = match self {
            Self::Off => return,
            Self::Ready(s) => s,
            Self::Pending(h) if h.is_finished() => match h.join() {
                Ok(s) => s,
                Err(_) => return,
            },
            Self::Pending(_) => return,
        };
        if let Some(line) = message(&status, env!("CARGO_PKG_VERSION")) {
            eprintln!("{line}");
        }
    }
}

/// The one-line notice, or nothing when up to date or unknown.
pub fn message(status: &Status, current: &str) -> Option<String> {
    match status {
        Status::Available(r) => Some(format!(
            "prybar {} is available (you have {current}): prybar self-update",
            r.version
        )),
        _ => None,
    }
}
