//! Crash reports: sanitize the panic text, keep prybar's own frames, and
//! write the report to the outbox from the panic hook.

mod hook;
mod sanitize;
#[cfg(test)]
mod tests;

pub use hook::install;
pub use sanitize::{frames, message};
