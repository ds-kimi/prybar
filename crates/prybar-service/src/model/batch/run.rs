use super::include::Job;
use super::target::{key, place, Placed};
use crate::model::{DecompileOptions, DecompileReport, Failure, ModelReport};
use crate::source::Batch;
use std::collections::HashSet;

/// State shared by every argument of one decompile call, so output
/// folders and included models are unique across the whole run.
#[derive(Default)]
pub struct Run {
    pub report: DecompileReport,
    pub placed: Placed,
    pub done: HashSet<String>,
}

/// One job per model of the batch that this run has not seen yet.
pub fn first_jobs(batch: &Batch, opts: &DecompileOptions, run: &mut Run) -> Vec<Job> {
    let mut jobs = Vec::new();
    for stem in batch.stems.iter().filter(|s| run.done.insert(key(s))) {
        let name = stem.rsplit(['/', '\\']).next().unwrap_or(stem);
        let at = (batch.root.as_str(), stem.as_str(), name);
        let dir = place(&opts.out, opts.flat, at, &mut run.placed);
        let stem = stem.clone();
        jobs.push(Job {
            stem,
            dir,
            from: None,
        });
    }
    jobs
}

type Outcome = Result<(ModelReport, Vec<String>), Failure>;

/// Files each outcome in the report and returns, per written model, the
/// `$includemodel` names it carries.
pub fn record(run: &mut Run, jobs: &[Job], results: Vec<Outcome>) -> Vec<(String, Vec<String>)> {
    let mut found = Vec::new();
    for (job, r) in jobs.iter().zip(results) {
        match r {
            Ok((m, inc)) => {
                found.push((job.stem.clone(), inc));
                run.report.models.push(m);
            }
            Err(f) => run.report.failures.push(f),
        }
    }
    found
}
