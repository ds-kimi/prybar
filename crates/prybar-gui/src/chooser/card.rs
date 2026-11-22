//! One design card: lifts on hover, sinks on press, rings on focus.

use std::time::Duration;

use gpui::{div, ease_out_quint, prelude::*, Animation, AnimationExt as _, Context, Div, Stateful};

use super::preview;
use crate::actions::MOD;
use crate::app::Root;
use crate::state::DesignKind;
use crate::theme;

fn frame(name: &'static str, ix: usize) -> Stateful<Div> {
    let p = theme::chooser();
    div()
        .id(name)
        .debug_selector(move || format!("card-{}", name.to_lowercase()))
        .tab_index(ix as isize)
        .w_72()
        .p_5()
        .flex()
        .flex_col()
        .gap_3()
        .rounded_xl()
        .border_1()
        .border_color(p.border)
        .bg(p.surface)
        .cursor_pointer()
        .hover(move |s| s.bg(p.hover).border_color(p.accent).shadow_lg())
        .active(move |s| s.bg(p.active))
        .focus_visible(move |s| s.border_2().border_color(p.accent))
}

pub fn card(kind: DesignKind, ix: usize, cx: &mut Context<Root>) -> impl IntoElement {
    let name = kind.name();
    let body = frame(name, ix)
        .on_click(cx.listener(move |this, _, _, cx| {
            this.state.update(cx, |s, cx| s.set_design(Some(kind), cx))
        }))
        .child(preview::preview(kind))
        .child(
            div()
                .text_lg()
                .child(format!("{name}  ·  {MOD}-{}", ix + 1)),
        )
        .child(
            div()
                .text_sm()
                .text_color(theme::chooser().muted)
                .child(kind.pitch()),
        );
    // Cards rise in one after the other so the screen never starts static.
    body.with_animation(
        ("card-in", ix),
        Animation::new(Duration::from_millis(350 + 120 * ix as u64)).with_easing(ease_out_quint()),
        |el, t| el.opacity(t).mt(gpui::px(24. * (1. - t))),
    )
}
