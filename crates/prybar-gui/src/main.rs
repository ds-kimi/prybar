//! Desktop interface for prybar.

// Release builds on Windows are a windowed app: no console window opens
// behind the GUI. Debug builds keep the console for logs.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod actions;
mod app;
mod cli;
mod designs;
mod sample;
mod state;
mod telemetry;
mod textures;
mod theme;
mod update;
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
