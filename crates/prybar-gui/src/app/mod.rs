//! Window setup: key bindings, launch options and the root view.

mod assets;
mod forms;
mod handlers;
mod pick;
mod publish_inputs;
mod root;
#[cfg(test)]
mod tests;
mod ui;

pub use forms::Forms;
pub use root::Root;

use gpui::{px, size, App, AppContext as _, Bounds, WindowAppearance, WindowBounds, WindowOptions};

use crate::actions::{self, Quit};
use crate::cli::Launch;
use crate::state::AppState;

pub fn run(launch: Launch) {
    gpui_platform::application()
        .with_assets(assets::Assets)
        .run(move |cx: &mut App| {
            cx.bind_keys(actions::bindings());
            cx.on_action(|_: &Quit, cx| cx.quit());
            let bounds = Bounds::centered(None, size(px(1200.), px(780.)), cx);
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            };
            let opened = cx.open_window(options, |window, cx| {
                let screen = launch.screen;
                let state = cx.new(|cx| initial_state(launch, cx));
                follow_appearance(state.read(cx).light, cx);
                start_on(&state, screen, cx);
                cx.new(|cx| Root::new(state, window, cx))
            });
            // A window that fails to open leaves nothing to show, so quit cleanly.
            if opened.is_err() {
                cx.quit();
            }
        });
}

/// Files load in the background and move to their own screen when done,
/// so the asked screen waits until loading has settled.
fn start_on(state: &gpui::Entity<AppState>, screen: Option<crate::state::Screen>, cx: &mut App) {
    let Some(screen) = screen else { return };
    let state = state.downgrade();
    let timer = cx
        .background_executor()
        .timer(std::time::Duration::from_secs(1));
    cx.spawn(async move |cx| {
        timer.await;
        let _ = state.update(cx, |s, cx| s.navigate(screen, cx));
    })
    .detach();
}

/// Applies the command line: files to open, and the sample.
fn initial_state(launch: Launch, cx: &mut gpui::Context<AppState>) -> AppState {
    let mut state = AppState::default();
    state.prefs = crate::state::prefs::default_path();
    state.light = state
        .prefs
        .as_deref()
        .is_some_and(crate::state::prefs::load_light);
    let mut files = launch.files;
    if launch.sample {
        match crate::sample::write(&crate::sample::default_dir()) {
            Ok(s) => files.extend([s.package, s.model]),
            Err(e) => state.toast(crate::state::ToastKind::Error, format!("Sample: {e}"), cx),
        }
    }
    for path in files {
        state.open(path, cx);
    }
    state
}

/// The title bar is drawn by macOS, so it follows the app theme only
/// through the app-wide appearance; other platforms ignore it.
pub fn follow_appearance(light: bool, cx: &App) {
    let look = if light {
        WindowAppearance::Light
    } else {
        WindowAppearance::Dark
    };
    cx.set_window_appearance(Some(look));
}
