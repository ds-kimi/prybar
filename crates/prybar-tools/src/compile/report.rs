use serde::Serialize;

/// One error or warning line from the studiomdl log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LogLine {
    pub line: usize,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompileReport {
    /// Exit code 0 *and* no error line: studiomdl sometimes exits 0
    /// after printing an ERROR and writing nothing.
    pub success: bool,
    pub exit_code: Option<i32>,
    pub errors: Vec<LogLine>,
    pub warnings: Vec<LogLine>,
    pub log: String,
}
