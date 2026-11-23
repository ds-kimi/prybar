//! The small prompt under the toolbar that asks for a link, an image
//! address or who is quoted, then writes the tag.

use gpui::{div, prelude::*, Context};

use super::super::tip::Tipped as _;
use super::super::{button, Kind};
use super::{Ask, BbEditor};
use crate::theme::prybar::Tokens;

impl BbEditor {
    /// A selected link is used as it is; otherwise the prompt opens.
    pub(super) fn ask(&mut self, ask: Ask, cx: &mut Context<Self>) {
        let picked = self.input.read(cx).selected().to_owned();
        if ask == Ask::Link && picked.starts_with("http") {
            self.input
                .update(cx, |i, cx| i.surround("[url]", "[/url]", cx));
            self.focus_input = true;
            return;
        }
        self.asking = Some(ask);
        self.value.update(cx, |v, cx| v.set_text("", cx));
        self.focus_value = true;
    }

    /// Writes the tag with the value; an empty link or image is no tag.
    pub(super) fn submit(&mut self, cx: &mut Context<Self>) {
        let Some(ask) = self.asking.take() else {
            return;
        };
        let v = self.value.read(cx).text().trim().to_owned();
        let picked = self.input.read(cx).selected().is_empty();
        let (open, close) = match ask {
            Ask::Link if v.is_empty() => return self.cancel(cx),
            Ask::Image if v.is_empty() => return self.cancel(cx),
            Ask::Link if picked => (format!("[url={v}]{v}"), "[/url]"),
            Ask::Link => (format!("[url={v}]"), "[/url]"),
            Ask::Image => (format!("[img]{v}[/img]"), ""),
            Ask::Quote if v.is_empty() => ("[quote]".into(), "[/quote]"),
            Ask::Quote => (format!("[quote={v}]"), "[/quote]"),
        };
        self.input.update(cx, |i, cx| i.surround(&open, close, cx));
        self.cancel(cx);
    }

    pub(super) fn cancel(&mut self, cx: &mut Context<Self>) {
        self.asking = None;
        self.focus_input = true;
        cx.notify();
    }

    pub(super) fn prompt(&self, ask: Ask, t: Tokens, cx: &mut Context<Self>) -> impl IntoElement {
        let label = match ask {
            Ask::Link => "Link address",
            Ask::Image => "Image address",
            Ask::Quote => "Who said it (optional)",
        };
        let insert = button("pb-bb-insert", "Insert", Kind::Primary, true, t)
            .tip("Write the tag into the description (Enter)", t)
            .on_click(cx.listener(|this, _, _, cx| this.submit(cx)));
        let cancel = button("pb-bb-cancel", "Cancel", Kind::Secondary, true, t)
            .tip("Close this prompt; nothing is written", t)
            .on_click(cx.listener(|this, _, _, cx| this.cancel(cx)));
        div()
            .debug_selector(|| "pb-bb-prompt".into())
            .flex()
            .items_center()
            .gap_2()
            .p_2()
            .rounded_md()
            .bg(t.hover)
            .child(div().text_xs().text_color(t.muted).flex_none().child(label))
            .child(div().flex_1().min_w_0().child(self.value.clone()))
            .child(insert)
            .child(cancel)
    }
}
