//! Home: a big friendly drop zone with a bobbing icon, then recent files
//! as cards.

use gpui::{div, prelude::*, AnyElement, Context, ExternalPaths};

use super::super::{motion, widgets};
use crate::actions::{Open, MOD};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::Palette;

pub fn view(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_6()
        .child(zone(s.documents.is_empty(), p))
        .child(widgets::title("Recent", "Pick up where you left off.", p))
        .child(recent(s, p, cx))
        .into_any_element()
}

/// The big target; it tints while files are dragged over it.
fn zone(idle: bool, p: Palette) -> AnyElement {
    let label = format!("Choose a file  {MOD}-o");
    let open = widgets::pill("cv-home-open", label, true, true, p)
        .debug_selector(|| "cv-home-open".into())
        .on_click(|_, window, cx| window.dispatch_action(Box::new(Open), cx));
    div()
        .id("cv-drop")
        .debug_selector(|| "cv-drop".into())
        .py_12()
        .flex()
        .flex_col()
        .items_center()
        .gap_4()
        .rounded_3xl()
        .border_2()
        .border_dashed()
        .border_color(p.border)
        .bg(p.surface)
        .hover(move |st| st.border_color(p.accent))
        .drag_over::<ExternalPaths>(move |st, _, _, _| st.bg(p.hover).border_color(p.accent))
        .child(icon(idle, p))
        .child(div().text_2xl().child("Drop a model or package here"))
        .child(div().text_color(p.muted).child(".mdl, .vpk, .gma and more"))
        .child(open)
        .into_any_element()
}

fn recent(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    if s.recent.is_empty() {
        return div()
            .text_color(p.muted)
            .child("Nothing yet. Your files will show up here.")
            .into_any_element();
    }
    let cards = s.recent.iter().enumerate().map(|(ix, path)| {
        let open = path.clone();
        let name = path.file_name().map_or_else(
            || path.display().to_string(),
            |n| n.to_string_lossy().into_owned(),
        );
        let card = widgets::card(("cv-recent", ix), p)
            .w_56()
            .flex()
            .flex_col()
            .gap_2()
            .on_click(cx.listener(move |this, _, _, cx| this.open_paths(vec![open.clone()], cx)))
            .child(div().h_16().rounded_xl().bg(p.hover))
            .child(div().text_sm().child(name));
        motion::enter(div().child(card), ("cv-recent-in", ix), ix)
    });
    div()
        .flex()
        .flex_wrap()
        .gap_4()
        .children(cards)
        .into_any_element()
}

/// The icon only bobs while nothing is open, so it invites the first drop.
fn icon(idle: bool, p: Palette) -> AnyElement {
    let icon = div()
        .size_16()
        .rounded_2xl()
        .bg(p.accent)
        .shadow_lg()
        .flex()
        .items_center()
        .justify_center()
        .text_2xl()
        .text_color(p.surface)
        .child("↓");
    if idle {
        return motion::idle(icon);
    }
    icon.into_any_element()
}
