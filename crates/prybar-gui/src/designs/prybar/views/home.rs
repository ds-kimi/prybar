//! Open screen: one drop target with the open button, then recent files.

use std::path::Path;

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::widgets::Tipped as _;
use super::super::widgets::{self, Kind};
use crate::actions::{Open, MOD};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .max_w(gpui::px(720.))
        .child(widgets::heading("Open"))
        .child(zone(t))
        .child(widgets::section("Recent", t))
        .child(recent(s, t, cx))
        .into_any_element()
}

fn zone(t: Tokens) -> impl IntoElement {
    let open = widgets::button("pb-open", "Open file", Kind::Primary, true, t)
        .debug_selector(|| "pb-open".into())
        .tip("Pick a model (.mdl) or a package (VPK, GMA) to open", t)
        .on_click(|_, window, cx| window.dispatch_action(Box::new(Open), cx));
    div()
        .id("pb-drop")
        .debug_selector(|| "pb-drop".into())
        .h_40()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_3()
        .rounded_lg()
        .border_1()
        .border_dashed()
        .border_color(t.border)
        .bg(t.surface)
        .text_color(t.muted)
        .hover(move |st| st.border_color(t.muted).text_color(t.text))
        .child("Drop a .mdl, .vpk, .gma, .apk or .hfs anywhere in the window")
        .child(open)
        .child(div().text_xs().child(format!("or press {MOD}-o")))
}

fn recent(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    if s.recent.is_empty() {
        return div()
            .text_color(t.muted)
            .child("Files you open show up here.")
            .into_any_element();
    }
    let rows = s.recent.iter().enumerate().map(|(ix, path)| {
        let open = path.clone();
        widgets::row(("pb-recent", ix), false, t)
            .debug_selector(move || format!("pb-recent-{ix}"))
            .tip("Open this file again", t)
            .on_click(cx.listener(move |this, _, _, cx| this.open_paths(vec![open.clone()], cx)))
            .child(split(path).0)
            .child(
                div()
                    .flex_1()
                    .truncate()
                    .text_xs()
                    .text_color(t.muted)
                    .child(split(path).1),
            )
    });
    div().flex().flex_col().children(rows).into_any_element()
}

/// File name and its folder, shown apart so the name leads the row.
fn split(path: &Path) -> (String, String) {
    let name = path.file_name().map_or_else(
        || path.display().to_string(),
        |n| n.to_string_lossy().into_owned(),
    );
    let folder = path.parent().map(|p| p.display().to_string());
    (name, folder.unwrap_or_default())
}
