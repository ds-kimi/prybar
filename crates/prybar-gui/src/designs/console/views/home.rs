//! Open: what is open now and what was open before, one row each.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::widgets;
use crate::actions::{Open, MOD};
use crate::app::Root;
use crate::state::{AppState, Document, Screen};
use crate::theme::Palette;

pub fn view(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let open = widgets::action("cs-home-open", "open file", &format!("{MOD}-o"), true, p)
        .on_click(|_, window, cx| window.dispatch_action(Box::new(Open), cx));
    div()
        .id("cs-drop")
        .debug_selector(|| "cs-drop".into())
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading(
            "open",
            "drop a file anywhere, or type its path",
            p,
        ))
        .child(div().flex().child(open))
        .child(widgets::heading(
            "files",
            format!("{} open", s.documents.len()),
            p,
        ))
        .child(files(s, p, cx))
        .child(widgets::heading(
            "recent",
            format!("{} kept", s.recent.len()),
            p,
        ))
        .child(recent(s, p, cx))
        .into_any_element()
}

fn files(s: &AppState, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let rows = s.documents.iter().enumerate().map(|(i, d)| {
        let (kind, screen) = match d {
            Document::Model { .. } => ("mdl", Screen::Inspector),
            Document::Package { .. } => ("pkg", Screen::Package),
        };
        let on = s.selected == Some(i);
        widgets::row(("cs-file", i), p)
            .debug_selector(move || format!("cs-file-{i}"))
            .when(on, |el| el.border_color(p.accent))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| {
                    s.select(i, cx);
                    s.navigate(screen, cx)
                })
            }))
            .child(div().w_8().text_color(p.muted).child(kind))
            .child(d.title())
    });
    div().flex().flex_col().children(rows)
}

fn recent(s: &AppState, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let rows = s.recent.iter().enumerate().map(|(i, path)| {
        let open = path.clone();
        widgets::row(("cs-recent", i), p)
            .on_click(cx.listener(move |this, _, _, cx| this.open_paths(vec![open.clone()], cx)))
            .child(div().w_8().text_color(p.muted).child(format!("{i}")))
            .child(path.display().to_string())
    });
    div().flex().flex_col().children(rows)
}
