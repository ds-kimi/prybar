//! The frame: command bar on top, a thin row of screen names, and the
//! footer line. The open bar's matches are in `list`.

pub mod foot;
pub mod list;

use gpui::{div, prelude::*, AnyElement, Context};

use super::{motion, widgets};
use crate::actions::{TogglePalette, MOD};
use crate::app::Root;
use crate::state::{AppState, Screen};
use crate::theme::Palette;

/// Always there. Closed it invites typing; open it shows the query, a
/// blinking caret, and flashes the prompt on every key.
pub fn bar(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let open = s.search.is_some();
    let query = s.search.clone().unwrap_or_default();
    let prompt = div().text_color(p.accent).child("›");
    let text = match open {
        true => div().child(query.clone()),
        false => div().text_color(p.muted).child("type a command or a path"),
    };
    div()
        .id("cs-bar")
        .debug_selector(|| "cs-bar".into())
        .h_10()
        .px_3()
        .flex()
        .items_center()
        .gap_2()
        .border_b_1()
        .border_color(if open { p.accent } else { p.border })
        .cursor_text()
        .hover(move |st| st.bg(p.surface))
        .on_click(
            cx.listener(|_, _, window, cx| window.dispatch_action(Box::new(TogglePalette), cx)),
        )
        .child(motion::flash(prompt, ("cs-type", query.len())))
        .child(text)
        .children(open.then(|| motion::caret(div().w_px().h_4().bg(p.accent))))
        .child(div().flex_1())
        .child(widgets::key(format!("{MOD}-k"), p))
        .into_any_element()
}

/// Screen names as a breadcrumb row, so the mouse can go anywhere too.
pub fn crumbs(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let items = Screen::ALL
        .into_iter()
        .map(|sc| crumb(sc, sc == s.screen, p, cx));
    div()
        .px_3()
        .py_1()
        .flex()
        .flex_wrap()
        .gap_1()
        .text_xs()
        .children(items)
        .into_any_element()
}

fn crumb(screen: Screen, on: bool, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let label = screen.label().to_lowercase();
    let sel = format!("nav-{label}");
    div()
        .id(screen.label())
        .debug_selector(move || sel.clone())
        .px_1()
        .cursor_pointer()
        .tab_index(0)
        .text_color(if on { p.text } else { p.muted })
        .when(on, |el| el.bg(p.active))
        .hover(move |st| st.text_color(p.text).bg(p.hover))
        .active(move |st| st.bg(p.active))
        .focus_visible(move |st| st.border_1().border_color(p.accent))
        .on_click(
            cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| s.navigate(screen, cx))
            }),
        )
        .child(label)
}
