//! One handler per command; each returns the exit code.

mod model;
mod package;
mod publish;
mod tools;

use crate::cli::Command;
use crate::output::Output;
use prybar_service::Result;

pub fn run(cmd: Command, out: &Output) -> u8 {
    let result: Result<u8> = match cmd {
        Command::Inspect(a) => model::inspect(&a, out),
        Command::Decompile(a) => model::decompile(&a, out),
        Command::List(a) => package::list(&a, out),
        Command::Unpack(a) => package::unpack(&a, out),
        Command::Pack(a) => package::pack(&a, out),
        Command::Compile(a) => tools::compile(&a, out),
        Command::View(a) => tools::view(&a, out),
        Command::Publish(a) => publish::publish(&a, out),
        Command::Games(a) => tools::games(&a, out),
        Command::Version(a) => crate::update::version(&a, out),
        Command::SelfUpdate => crate::update::self_update(out),
    };
    result.unwrap_or_else(|e| out.error(&e))
}

/// Human output for reports that have no dedicated layout.
fn pretty<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).map_or_else(|e| format!("{e}\n"), |s| s + "\n")
}
