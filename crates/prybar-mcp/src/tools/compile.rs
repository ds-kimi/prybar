use crate::guard;
use crate::params::CompileArgs;
use prybar_service::tools::{self, CompileBatchReport, CompileOptions};
use prybar_service::{Result, ServiceError};
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

/// Same ceiling as decompile inputs: one call stays a bounded job.
const MAX_INPUTS: usize = 64;
/// Items past this are dropped from the reply (counts stay exact) so a
/// big folder cannot flood the context window.
const SHOWN: usize = 200;

pub enum Plan {
    One(String, PathBuf),
    Many(String, Vec<String>, CompileOptions),
}

/// `qc` alone keeps the single report with the log; anything else is a
/// batch, so the old call shape behaves exactly as before.
pub fn plan(a: CompileArgs) -> Result<Plan> {
    if let (Some(qc), true, false) = (&a.qc, a.inputs.is_empty(), a.recursive) {
        return Ok(Plan::One(a.game, guard::absolute("qc", qc)?));
    }
    let inputs: Vec<String> = a.qc.into_iter().chain(a.inputs).collect();
    if inputs.is_empty() || inputs.len() > MAX_INPUTS {
        return Err(ServiceError::Usage(format!(
            "qc or inputs: expected 1 to {MAX_INPUTS} paths"
        )));
    }
    for (i, input) in inputs.iter().enumerate() {
        guard::absolute(&format!("inputs[{i}]"), input)?;
    }
    let opts = CompileOptions {
        recursive: a.recursive,
        jobs: a.jobs.map(|j| j.min(16)),
    };
    Ok(Plan::Many(a.game, inputs, opts))
}

pub fn batch(game: &str, inputs: &[String], opts: &CompileOptions) -> Result<CompileBatchReport> {
    let never = AtomicBool::new(false);
    let mut r = tools::compile_batch(game, inputs, opts, &|_| {}, &never)?;
    // Failures first so truncation never hides what needs fixing.
    r.items.sort_by_key(|i| i.code == 0);
    r.items.truncate(SHOWN);
    Ok(r)
}
