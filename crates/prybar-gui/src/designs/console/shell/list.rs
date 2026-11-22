//! The open bar's matches, dropped under it. The highlight springs to
//! the row under the cursor; up/down move it, enter runs it.

use gpui::{div, prelude::*, px, AnyElement, Context};

use super::super::commands::{self, Command};
use super::super::widgets::{self, ROW};
use super::super::{motion, run};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::Palette;

pub fn drop(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let cmds = commands::matches(s);
    let cursor = s.cursor.min(cmds.len().saturating_sub(1));
    let rows: Vec<_> = cmds
        .into_iter()
        .enumerate()
        .map(|(ix, c)| item(ix, c, ix == cursor, p, cx))
        .collect();
    let empty = rows.is_empty();
    let mark = div().absolute().left_0().w_full().h(px(ROW)).bg(p.active);
    div()
        .id("cs-list")
        .debug_selector(|| "cs-list".into())
        .absolute()
        .top_10()
        .left_0()
        .right_0()
        .bg(p.surface)
        .border_b_1()
        .border_color(p.accent)
        .shadow_lg()
        .when(!empty, |el| {
            el.child(motion::slide_to(mark, "cs-mark", cursor as f32 * ROW))
        })
        .when(empty, |el| {
            el.child(div().px_3().py_2().text_color(p.muted).child("no match"))
        })
        .children(rows)
        .into_any_element()
}

fn item(ix: usize, c: Command, on: bool, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let act = c.act.clone();
    widgets::row(("cs-cmd", ix), p)
        .debug_selector(move || format!("cs-cmd-{ix}"))
        .when(on, |el| el.border_color(p.accent))
        .on_click(cx.listener(move |this, _, window, cx| run::pick(act.clone(), this, window, cx)))
        .child(
            div()
                .flex_1()
                .text_color(if on { p.text } else { p.muted })
                .child(c.label),
        )
        .child(div().text_xs().text_color(p.muted).child(c.hint))
        .children(on.then(|| widgets::key("⏎", p)))
}
