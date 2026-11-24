use prybar_tools::compile::{CompileReport, LogLine};
use serde::Serialize;

/// How a batch compile picks and runs its QCs.
#[derive(Debug, Clone, Default)]
pub struct CompileOptions {
    /// Descend into subfolders of folder and wildcard inputs.
    pub recursive: bool,
    /// studiomdl runs at once; `None` = half the cores, since each run
    /// is heavy and they all write into the same game folder.
    pub jobs: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemStatus {
    Ok,
    Failed,
    Cancelled,
}

/// One QC of a batch. The full log is left out: a folder of QCs would
/// make the report megabytes; the error lines are what fixes a QC.
#[derive(Debug, Clone, Serialize)]
pub struct CompileItem {
    pub qc: String,
    pub status: ItemStatus,
    /// Exit code the item maps to (6 compile failed, 3 not found...).
    pub code: u8,
    pub exit_code: Option<i32>,
    pub errors: Vec<LogLine>,
    pub warnings: Vec<LogLine>,
    pub outputs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CompileBatchReport {
    pub compiled: usize,
    pub failed: usize,
    pub cancelled: usize,
    pub items: Vec<CompileItem>,
}

impl CompileItem {
    pub fn from_report(qc: String, r: CompileReport, cancelled: bool) -> Self {
        let status = match (r.success, cancelled) {
            (true, _) => ItemStatus::Ok,
            (false, true) => ItemStatus::Cancelled,
            (false, false) => ItemStatus::Failed,
        };
        let code = if r.success {
            0
        } else {
            crate::ExitCode::Tool as u8
        };
        CompileItem {
            qc,
            status,
            code,
            exit_code: r.exit_code,
            errors: r.errors,
            warnings: r.warnings,
            outputs: r.outputs,
            error: None,
        }
    }

    /// An item that never ran: bad input, missing file, or cancelled.
    pub fn without_run(qc: String, status: ItemStatus, code: u8, error: String) -> Self {
        CompileItem {
            qc,
            status,
            code,
            exit_code: None,
            errors: Vec::new(),
            warnings: Vec::new(),
            outputs: Vec::new(),
            error: Some(error),
        }
    }

    /// A QC that could not be run: missing input or studiomdl would not
    /// start.
    pub fn bad_input((qc, e): (String, crate::ServiceError)) -> Self {
        let code = e.exit_code() as u8;
        Self::without_run(qc, ItemStatus::Failed, code, e.to_string())
    }
}
