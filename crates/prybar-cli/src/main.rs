//! `prybar`: Source engine model and package tool for people and agents.

mod cli;
mod output;
mod run;
mod update;

use clap::Parser;

fn main() -> std::process::ExitCode {
    let cli = cli::Cli::parse();
    let out = output::Output {
        json: cli.json,
        quiet: cli.quiet,
    };
    let notice = update::Notice::start(&cli.command, &out);
    let code = run::run(cli.command, &out);
    notice.finish();
    std::process::ExitCode::from(code)
}
