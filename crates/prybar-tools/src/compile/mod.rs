//! studiomdl wrapper: run a QC compile and turn the console log into
//! structured errors and warnings.

mod args;
mod log;
mod report;
mod run;
mod stream;

pub use args::compile_command;
pub use log::{parse_log, written_files};
pub use report::{CompileReport, LogLine};
pub use run::compile;
pub use stream::compile_streamed;
