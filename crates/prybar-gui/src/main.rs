//! Desktop interface for prybar.

mod actions;
mod app;
mod cli;
mod designs;
mod sample;
mod state;
mod textures;
mod theme;
mod viewport;

fn main() {
    match cli::parse(std::env::args().skip(1)) {
        Ok(launch) => app::run(launch),
        Err(e) => {
            eprintln!("prybar-gui: {e}");
            std::process::exit(2);
        }
    }
}
