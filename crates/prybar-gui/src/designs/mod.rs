//! Design views. Workbench and Canvas have their own; Console shares this stub
//! (same screens and data, in their palette) until their lots land.

mod canvas;
mod content;
mod placeholder;
mod side;
mod workbench;

use gpui::{div, prelude::*, AnyElement, Context, Entity};

use crate::app::Root;
use crate::state::{AppState, DesignKind, Screen};
use crate::theme::{self, Palette};

pub fn render(kind: DesignKind, state: Entity<AppState>, cx: &mut Context<Root>) -> AnyElement {
    match kind {
        DesignKind::Workbench => workbench::render(state, cx),
        DesignKind::Canvas => canvas::render(state, cx),
        _ => stub(kind, state, cx),
    }
}

fn stub(kind: DesignKind, state: Entity<AppState>, cx: &mut Context<Root>) -> AnyElement {
    let p = theme::for_design(kind);
    let state = state.read(cx);
    let current = state.screen;
    let lines = content::lines(state);
    let header = side::header(kind, state);
    let toasts = side::toasts(state, p);
    let titles: Vec<String> = state.documents.iter().map(|d| d.title()).collect();
    let docs = side::documents(titles, state.selected, p, cx);
    let nav: Vec<_> = Screen::ALL
        .into_iter()
        .map(|s| nav_item(s, s == current, p, cx))
        .collect();
    let sidebar = div().w_48().p_3().flex().flex_col().gap_1().bg(p.surface);
    div()
        .size_full()
        .flex()
        .child(sidebar.children(nav).child(docs))
        .child(column(vec![
            div().text_lg().child(header).into_any_element(),
            content::body(current, lines, p, cx),
            toasts,
        ]))
        .into_any_element()
}

fn nav_item(screen: Screen, current: bool, p: Palette, cx: &mut Context<Root>) -> impl IntoElement {
    let label = screen.label();
    div()
        .id(label)
        .debug_selector(move || format!("nav-{}", label.to_lowercase()))
        .px_3()
        .py_1()
        .rounded_md()
        .cursor_pointer()
        .when(current, |el| el.bg(p.active).text_color(p.accent))
        .hover(move |s| s.bg(p.hover))
        .active(move |s| s.bg(p.active))
        .on_click(
            cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| s.navigate(screen, cx))
            }),
        )
        .child(label)
}

fn column(children: Vec<AnyElement>) -> impl IntoElement {
    div()
        .flex_1()
        .p_6()
        .flex()
        .flex_col()
        .gap_2()
        .children(children)
}
