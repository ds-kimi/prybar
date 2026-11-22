//! `prybar`: Source engine model and package tool for people and agents.

mod cli;
mod output;
mod run;

use clap::Parser;

fn main() -> std::process::ExitCode {
    let cli = cli::Cli::parse();
    let out = output::Output {
        json: cli.json,
        quiet: cli.quiet,
    };
    std::process::ExitCode::from(run::run(cli.command, &out))
}
