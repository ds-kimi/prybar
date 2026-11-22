//! studiomdl wrapper: run a QC compile and turn the console log into
//! structured errors and warnings.

mod args;
mod log;
mod report;
mod run;

pub use args::compile_command;
pub use log::parse_log;
pub use report::{CompileReport, LogLine};
pub use run::compile;
