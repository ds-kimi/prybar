//! The dimmed backdrop, the card and its enter and exit motion.

use std::time::Duration;

use gpui::{
    div, ease_out_quint, prelude::*, Animation, AnimationExt as _, Context, FontWeight,
    MouseButton, Pixels, Size, Window,
};

use super::super::tip::Tipped as _;
use super::super::{button, icon_button, Kind};
use super::Modal;
use crate::theme::prybar::{tokens, Light, Tokens};

impl Render for Modal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(cx.try_global::<Light>().is_some_and(|l| l.0));
        let card = self.card(window.viewport_size(), t, cx);
        let (from, to) = if self.closing { (1., 0.) } else { (0., 1.) };
        div()
            .id("pb-modal-backdrop")
            .debug_selector(|| "pb-modal-backdrop".into())
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::black().opacity(0.45))
            .occlude()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.request_close(cx)),
            )
            .child(card)
            .with_animation(
                if self.closing {
                    "pb-modal-out"
                } else {
                    "pb-modal-in"
                },
                Animation::new(Duration::from_millis(140)).with_easing(ease_out_quint()),
                move |el, k| el.opacity(from + (to - from) * k),
            )
    }
}

impl Modal {
    /// The body scrolls on its own so a small window never hides the
    /// header, its close button or the actions pinned at the bottom.
    fn card(&self, window: Size<Pixels>, t: Tokens, cx: &mut Context<Self>) -> impl IntoElement {
        let size = super::card_size(window);
        let body = div()
            .id("pb-modal-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .p_4()
            .child(self.body.clone());
        let card = div()
            .id("pb-modal")
            .debug_selector(|| "pb-modal".into())
            .track_focus(&self.focus)
            .tab_group()
            .occlude()
            .w(size.width)
            .max_h(size.height)
            .flex()
            .flex_col()
            .rounded_lg()
            .bg(t.raised)
            .border_1()
            .border_color(t.border)
            .shadow_lg();
        Self::keys(card, cx)
            .child(self.header(t, cx))
            .children(self.confirming.then(|| self.confirm(t, cx)))
            .child(body)
            .children(self.footer_bar(t))
    }

    fn header(&self, t: Tokens, cx: &mut Context<Self>) -> impl IntoElement {
        let close = icon_button("pb-modal-close", "✕", t)
            .debug_selector(|| "pb-modal-close".into())
            .tip("Close this popup (Escape)", t)
            .on_click(cx.listener(|this, _, _, cx| this.request_close(cx)));
        div()
            .px_4()
            .h_12()
            .flex()
            .items_center()
            .justify_between()
            .border_b_1()
            .border_color(t.border)
            .child(
                div()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.title.clone()),
            )
            .child(close)
    }

    /// Asked in place, not in a second popup, so the edits stay in sight.
    fn confirm(&self, t: Tokens, cx: &mut Context<Self>) -> impl IntoElement {
        let keep = button("pb-modal-keep", "Keep editing", Kind::Secondary, true, t)
            .debug_selector(|| "pb-modal-keep".into())
            .tip("Go back to the form; nothing is lost", t)
            .on_click(cx.listener(|this, _, _, cx| {
                this.confirming = false;
                cx.notify()
            }));
        let discard = button("pb-modal-discard", "Discard", Kind::Primary, true, t)
            .debug_selector(|| "pb-modal-discard".into())
            .tip("Close and throw away what you changed", t)
            .on_click(cx.listener(|this, _, _, cx| this.close(cx)));
        div()
            .debug_selector(|| "pb-modal-confirm".into())
            .px_4()
            .py_2()
            .flex()
            .items_center()
            .gap_2()
            .bg(t.hover)
            .child(div().flex_1().child("Discard your changes?"))
            .child(keep)
            .child(discard)
    }
}
