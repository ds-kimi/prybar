use prybar_service::ServiceError;
use serde::Serialize;

/// `--json` prints exactly one JSON document on stdout per run, so an
/// agent can parse it whole; progress and human text never mix in.
pub struct Output {
    pub json: bool,
    pub quiet: bool,
}

impl Output {
    pub fn emit<T: Serialize>(&self, value: &T, text: impl FnOnce(&T) -> String) {
        if self.json {
            match serde_json::to_string_pretty(value) {
                Ok(s) => println!("{s}"),
                Err(e) => eprintln!("error: cannot encode output: {e}"),
            }
        } else if !self.quiet {
            print!("{}", text(value));
        }
    }

    /// Progress goes to stderr so it never corrupts stdout.
    pub fn progress(&self, msg: &str) {
        if !self.quiet {
            eprintln!("{msg}");
        }
    }

    /// Prints the error and returns its exit code.
    pub fn error(&self, e: &ServiceError) -> u8 {
        let code = e.exit_code() as u8;
        if self.json {
            let doc = serde_json::json!({ "error": { "message": e.to_string(), "code": code } });
            println!("{doc}");
        } else {
            eprintln!("error: {e}");
        }
        code
    }
}
