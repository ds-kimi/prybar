//! Command palette (`mod-k`): every screen and window command in one list.

use gpui::{div, prelude::*, Action, AnyElement, Context};

use super::super::widgets::Tipped as _;
use super::super::{motion, widgets};
use crate::actions::{Open, ToggleTheme};
use crate::app::Root;
use crate::state::Screen;
use crate::theme::prybar::Tokens;

#[derive(Clone, Copy)]
enum Command {
    Go(Screen),
    Run(&'static str, fn() -> Box<dyn Action>),
}

fn commands() -> Vec<Command> {
    let mut out: Vec<Command> = Screen::ALL.into_iter().map(Command::Go).collect();
    out.extend([
        Command::Run("Open file", || Box::new(Open)),
        Command::Run("Switch theme", || Box::new(ToggleTheme)),
    ]);
    out
}

/// A click on the dimmed backdrop closes it, like Escape; clicks inside
/// the panel stop there.
pub fn overlay(t: Tokens, cx: &Context<Root>) -> AnyElement {
    let rows = commands()
        .into_iter()
        .enumerate()
        .map(|(ix, cmd)| row(ix, cmd, t, cx));
    let panel = panel(rows.collect(), t);
    div()
        .id("pb-palette-backdrop")
        .absolute()
        .inset_0()
        .pt_20()
        .flex()
        .justify_center()
        .items_start()
        .bg(gpui::hsla(0., 0., 0., 0.4))
        .on_click(cx.listener(|this, _, _, cx| this.state.update(cx, |s, cx| s.back(cx))))
        .child(motion::enter(div().child(panel), "pb-palette-in"))
        .into_any_element()
}

fn panel(rows: Vec<AnyElement>, t: Tokens) -> impl IntoElement {
    div()
        .id("pb-palette")
        .debug_selector(|| "pb-palette".into())
        .w_96()
        .p_1()
        .flex()
        .flex_col()
        .rounded_lg()
        .border_1()
        .border_color(t.border)
        .bg(t.raised)
        .shadow_lg()
        .on_click(|_, _, cx| cx.stop_propagation())
        .children(rows)
}

fn row(ix: usize, cmd: Command, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let label = match cmd {
        Command::Go(screen) => format!("Go to {}", screen.label()),
        Command::Run(label, _) => label.to_owned(),
    };
    widgets::row(("pb-cmd", ix), false, t)
        .debug_selector(move || format!("cmd-{ix}"))
        .tip("Run this command (Enter)", t)
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
        .into_any_element()
}
