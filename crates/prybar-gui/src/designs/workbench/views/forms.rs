//! Compile, publish, settings and the preview screens. The parts that
//! are later work are animated placeholders inside a real layout.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::super::placeholder;
use super::super::widgets;
use crate::actions::ToggleTheme;
use crate::app::Root;
use crate::state::{AppState, Placeholder};
use crate::theme::Palette;

/// Games offered by the compile picker until games config is wired.
const GAMES: [&str; 4] = [
    "Garry's Mod",
    "Team Fortress 2",
    "Counter-Strike: Source",
    "Half-Life 2",
];

pub fn compile(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().text_lg().child("Compile"))
        .child(
            div()
                .max_w_96()
                .child(widgets::field("wb-qc", "QC file", "No QC chosen", p)),
        )
        .child(widgets::section("Game", p))
        .child(games(s, p, cx))
        .child(
            div()
                .flex()
                .child(widgets::button("wb-compile", "Compile", true, false, p)),
        )
        .child(placeholder::region(Placeholder::CompileLog, p))
        .into_any_element()
}

/// The picked game stays highlighted in the accent.
fn games(s: &AppState, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let rows = GAMES.iter().enumerate().map(|(ix, name)| {
        widgets::row(("wb-game", ix), s.game == ix, p)
            .when(s.game == ix, |el| el.bg(p.active).text_color(p.accent))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| {
                    s.game = ix;
                    cx.notify();
                })
            }))
            .child(*name)
    });
    div().max_w_96().flex().flex_col().children(rows)
}

pub fn publish(s: &AppState, p: Palette) -> AnyElement {
    let title = s
        .selected
        .and_then(|i| s.documents.get(i))
        .map_or_else(|| "Untitled".into(), |d| d.title());
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(div().text_lg().child("Workshop publish"))
        .child(
            div()
                .max_w_96()
                .flex()
                .flex_col()
                .gap_3()
                .child(widgets::field("wb-pub-title", "Title", title, p))
                .child(widgets::field("wb-pub-vis", "Visibility", "Private", p))
                .child(widgets::field("wb-pub-tags", "Tags", "model", p)),
        )
        .child(
            div()
                .flex()
                .gap_2()
                .child(widgets::button("wb-pub-dry", "Dry run", false, false, p))
                .child(widgets::button("wb-pub-go", "Publish", true, false, p)),
        )
        .child(placeholder::region(Placeholder::PublishProgress, p))
        .into_any_element()
}

pub fn settings(s: &AppState, p: Palette) -> AnyElement {
    let theme = widgets::toggle("wb-light", "Light theme", s.light, p)
        .debug_selector(|| "wb-light".into())
        .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleTheme), cx));
    div()
        .flex()
        .flex_col()
        .gap_2()
        .max_w_96()
        .child(div().text_lg().child("Settings"))
        .child(widgets::section("Appearance", p))
        .child(theme)
        .child(widgets::section("Games", p))
        .child(widgets::field(
            "wb-games",
            "Games config",
            "Detected games appear here",
            p,
        ))
        .child(div().flex().child(widgets::button(
            "wb-detect",
            "Detect games",
            false,
            false,
            p,
        )))
        .into_any_element()
}
