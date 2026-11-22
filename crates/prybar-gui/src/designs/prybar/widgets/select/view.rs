//! The closed button and the open list, drawn above everything else so
//! no panel clips it.

use gpui::{anchored, deferred, div, prelude::*, px, AnyElement, Context, FontWeight, Window};

use super::super::ring;
use super::super::tip::Tipped as _;
use super::{Choice, Select};
use crate::theme::prybar::{tokens, Light, Tokens};

impl Render for Select {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(cx.try_global::<Light>().is_some_and(|l| l.0));
        div()
            .relative()
            .child(self.button(t, cx))
            .when(self.open, |el| el.child(self.list(t, cx)))
    }
}

impl Select {
    fn button(&self, t: Tokens, cx: &mut Context<Self>) -> impl IntoElement {
        let id = self.id.clone();
        let tip = self.choices.get(self.current).map(|c| c.tip.clone());
        let arrow = if self.open { "▴" } else { "▾" };
        ring(div().id(id.clone()), t)
            .debug_selector(move || id.to_string())
            .track_focus(&self.focus)
            .h_8()
            .px_2()
            .flex()
            .items_center()
            .justify_between()
            .rounded_md()
            .bg(t.surface)
            .border_color(t.border)
            .cursor_pointer()
            .hover(move |s| s.bg(t.hover))
            .active(move |s| s.bg(t.active))
            .tip(tip.unwrap_or_default(), t)
            .on_key_down(cx.listener(Self::key))
            .on_click(cx.listener(|this, _, w, cx| {
                w.focus(&this.focus, cx);
                this.toggle(cx)
            }))
            .child(self.label())
            .child(div().text_color(t.muted).child(arrow))
    }

    /// A click anywhere else closes it, like a native menu.
    fn list(&self, t: Tokens, cx: &mut Context<Self>) -> AnyElement {
        let rows = self.choices.iter().enumerate();
        let list = div()
            .id("pb-select-list")
            .occlude()
            .mt_1()
            .min_w(px(180.))
            .p_1()
            .rounded_lg()
            .bg(t.raised)
            .border_1()
            .border_color(t.border)
            .shadow_md()
            .on_mouse_down_out(cx.listener(|this, _, _, cx| (this.open = false, cx.notify()).1))
            .children(rows.map(|(i, c)| self.option(i, c, t, cx)));
        deferred(anchored().child(list))
            .with_priority(1)
            .into_any_element()
    }

    /// The check marks the current choice; the tint follows the keys.
    fn option(&self, i: usize, c: &Choice, t: Tokens, cx: &mut Context<Self>) -> impl IntoElement {
        let (on, hi) = (i == self.current, i == self.hi);
        let id = format!("{}-opt-{i}", self.id);
        let mark = div()
            .w(px(14.))
            .text_color(t.rust)
            .child(if on { "✓" } else { "" });
        let text = div().when(on, |el| el.font_weight(FontWeight::MEDIUM));
        div()
            .id(i)
            .debug_selector(move || id.clone())
            .h_8()
            .px_2()
            .flex()
            .items_center()
            .gap_2()
            .rounded_md()
            .cursor_pointer()
            .when(hi, |el| el.bg(t.hover))
            .hover(move |s| s.bg(t.hover))
            .active(move |s| s.bg(t.active))
            .tip(c.tip.clone(), t)
            .on_click(cx.listener(move |this, _, _, cx| this.pick(i, cx)))
            .child(mark)
            .child(text.child(c.label.clone()))
    }
}
