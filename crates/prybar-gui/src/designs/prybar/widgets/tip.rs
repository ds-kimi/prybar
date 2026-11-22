//! One tooltip for the whole app: a short plain sentence on a raised
//! card, after a short pause so moving the mouse across the UI stays
//! quiet. GPUI keeps it inside the window, flipping it near the edges.

use std::time::Duration;

use gpui::{
    div, ease_out_quint, prelude::*, px, Animation, AnimationExt as _, Context, SharedString,
    Window,
};

use crate::theme::prybar::Tokens;

/// Long enough that a passing mouse does not pop cards everywhere.
pub const DELAY: Duration = Duration::from_millis(400);

pub struct Tip {
    pub text: SharedString,
    t: Tokens,
}

/// Release notes land in the update button's tip too, hence the width
/// cap that wraps instead of stretching across the window.
///
/// Any clickable or hoverable element can explain itself.
pub trait Tipped: StatefulInteractiveElement + Sized {
    fn tip(self, text: impl Into<SharedString>, t: Tokens) -> Self {
        let text = text.into();
        self.tooltip(move |_, cx| {
            let text = text.clone();
            cx.new(|_| Tip { text, t }).into()
        })
        .tooltip_show_delay(DELAY)
    }
}

impl<E: StatefulInteractiveElement + Sized> Tipped for E {}

impl Render for Tip {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let t = self.t;
        div()
            .debug_selector(|| "pb-tip".into())
            .pl_3()
            .pt_4()
            .child(
                div()
                    .max_w(px(360.))
                    .px_2()
                    .py_1()
                    .rounded_lg()
                    .bg(t.raised)
                    .border_1()
                    .border_color(t.border)
                    .shadow_md()
                    .text_xs()
                    .text_color(t.text)
                    .child(self.text.clone()),
            )
            .with_animation(
                "pb-tip",
                Animation::new(Duration::from_millis(120)).with_easing(ease_out_quint()),
                |el, k| el.opacity(k),
            )
    }
}
