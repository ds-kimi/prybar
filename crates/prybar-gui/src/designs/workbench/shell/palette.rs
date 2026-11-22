//! Command palette: every screen and window command in one list.

use gpui::{div, prelude::*, Action, AnyElement, Context};

use super::super::{motion, widgets};
use crate::actions::{Open, ShowCanvas, ShowChooser, ShowConsole, ToggleTheme};
use crate::app::Root;
use crate::state::Screen;
use crate::theme::Palette;

#[derive(Clone, Copy)]
enum Command {
    Go(Screen),
    Run(&'static str, fn() -> Box<dyn Action>),
}

fn commands() -> Vec<Command> {
    let mut out: Vec<Command> = Screen::ALL.into_iter().map(Command::Go).collect();
    out.extend([
        Command::Run("Open file", || Box::new(Open)),
        Command::Run("Toggle light theme", || Box::new(ToggleTheme)),
        Command::Run("Switch to Canvas", || Box::new(ShowCanvas)),
        Command::Run("Switch to Console", || Box::new(ShowConsole)),
        Command::Run("Back to the chooser", || Box::new(ShowChooser)),
    ]);
    out
}

/// A click outside the panel closes it, like Escape.
pub fn overlay(p: Palette, cx: &Context<Root>) -> AnyElement {
    let rows: Vec<_> = commands()
        .into_iter()
        .enumerate()
        .map(|(ix, cmd)| row(ix, cmd, p, cx).into_any_element())
        .collect();
    div()
        .id("wb-palette-backdrop")
        .absolute()
        .inset_0()
        .pt_20()
        .flex()
        .justify_center()
        .items_start()
        .bg(gpui::hsla(0., 0., 0., 0.35))
        .on_click(cx.listener(|this, _, _, cx| this.state.update(cx, |s, cx| s.back(cx))))
        .child(motion::enter(div().child(panel(rows, p)), "wb-palette-in"))
        .into_any_element()
}

/// Clicks inside the panel stop here so they never reach the backdrop.
fn panel(rows: Vec<AnyElement>, p: Palette) -> impl IntoElement {
    div()
        .id("wb-palette")
        .debug_selector(|| "wb-palette".into())
        .w_96()
        .p_1()
        .flex()
        .flex_col()
        .rounded_lg()
        .border_1()
        .border_color(p.border)
        .bg(p.surface)
        .shadow_lg()
        .on_click(|_, _, cx| cx.stop_propagation())
        .child(widgets::section("Commands", p))
        .children(rows)
}

fn row(ix: usize, cmd: Command, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let label = match cmd {
        Command::Go(screen) => format!("Go to {}", screen.label()),
        Command::Run(label, _) => label.to_owned(),
    };
    widgets::row(("wb-cmd", ix), false, p)
        .debug_selector(move || format!("cmd-{ix}"))
        .on_click(cx.listener(move |this, _, window, cx| {
            this.state.update(cx, |s, cx| {
                s.palette = false;
                if let Command::Go(screen) = cmd {
                    s.navigate(screen, cx);
                }
                cx.notify();
            });
            if let Command::Run(_, action) = cmd {
                window.dispatch_action(action(), cx);
            }
        }))
        .child(label)
}
