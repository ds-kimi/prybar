//! Open screen: a drop zone, the open button and recent files.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::widgets;
use crate::actions::{Open, MOD};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::Palette;

pub fn view(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let open = widgets::button("wb-home-open", format!("Open…  {MOD}-o"), true, true, p)
        .debug_selector(|| "wb-home-open".into())
        .on_click(|_, window, cx| window.dispatch_action(Box::new(Open), cx));
    let zone = div()
        .id("wb-drop")
        .h_40()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_3()
        .rounded_lg()
        .border_1()
        .border_dashed()
        .border_color(p.border)
        .text_color(p.muted)
        .hover(move |st| st.border_color(p.accent).text_color(p.text))
        .child("Drop a .mdl, .vpk or .gma anywhere in the window")
        .child(open);
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().text_lg().child("Open"))
        .child(zone)
        .child(widgets::section("Recent", p))
        .child(recent(s, p, cx))
        .into_any_element()
}

fn recent(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    if s.recent.is_empty() {
        return div()
            .px_2()
            .text_color(p.muted)
            .child("Nothing yet")
            .into_any_element();
    }
    let rows = s.recent.iter().enumerate().map(|(ix, path)| {
        let open = path.clone();
        widgets::row(("wb-recent", ix), false, p)
            .on_click(cx.listener(move |this, _, _, cx| this.open_paths(vec![open.clone()], cx)))
            .child(path.display().to_string())
    });
    div().flex().flex_col().children(rows).into_any_element()
}
