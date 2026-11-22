//! Layout: material list on the left; on the right the controls, the
//! preview and the facts about the shown texture.

use gpui::{div, prelude::*, Context, Div, Window};

use super::{canvas, facts, input, side, Channel, TextureView};
use crate::theme::prybar::{tokens, Light, Tokens};

impl Render for TextureView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(cx.try_global::<Light>().is_some_and(|l| l.0));
        let focus = self.focus.get_or_insert_with(|| cx.focus_handle()).clone();
        let surface = canvas::surface(self, cx.entity());
        let preview = input::attach(div(), cx)
            .track_focus(&focus)
            .debug_selector(|| "pb-tex-preview".into())
            .relative()
            .flex_1()
            .min_h_48()
            .rounded_lg()
            .border_1()
            .border_color(t.border)
            .bg(t.surface)
            .overflow_hidden()
            .child(surface.absolute().size_full())
            .children(side::message(self, &t));
        let right = div().flex_1().min_w_0().flex().flex_col().gap_2();
        let right = right
            .child(controls(self, &t, cx))
            .child(preview)
            .child(details(self, &t));
        div()
            .size_full()
            .flex()
            .gap_4()
            .child(side::list(self, &t, cx))
            .child(right)
    }
}

type Act = fn(&mut TextureView, &mut Context<TextureView>);

/// Channel, mip and fit chips, in screen order; the mip label sits
/// between the two mip chips.
const CHIPS: [(&str, &str, Act); 5] = [
    ("pb-tex-rgb", "RGB", |v, cx| {
        (v.channel = Channel::Rgb, v.refresh(cx)).1
    }),
    ("pb-tex-alpha", "Alpha", |v, cx| {
        (v.channel = Channel::Alpha, v.refresh(cx)).1
    }),
    ("pb-tex-mip-prev", "−", |v, cx| {
        v.set_mip(v.mip.saturating_sub(1), cx)
    }),
    ("pb-tex-mip-next", "+", |v, cx| {
        v.set_mip(v.mip.saturating_add(1), cx)
    }),
    ("pb-tex-fit", "Fit", |v, _| v.pan = Default::default()),
];

fn controls(v: &TextureView, t: &Tokens, cx: &mut Context<TextureView>) -> Div {
    let slot = v.set.as_ref().and_then(|s| s.slots.get(v.selected));
    let mips = slot.and_then(|s| s.info.as_ref()).map_or(1, |i| i.mips);
    let rgb = v.channel == Channel::Rgb;
    let on = [rgb, !rgb, false, false, v.pan.scale.is_none()];
    let mut chips = CHIPS
        .iter()
        .zip(on)
        .map(|(&(id, label, act), on)| side::chip(id, label, on, t, cx, act));
    let label = format!("Mip {} of {}", v.mip, mips - 1);
    let label = div().px_1().text_xs().text_color(t.muted).child(label);
    let row = div()
        .flex()
        .gap_1()
        .items_center()
        .children(chips.by_ref().take(3));
    row.child(label).children(chips)
}

/// Size, format and source of the shown texture, or where it was looked for.
fn details(v: &TextureView, t: &Tokens) -> Div {
    let slot = v.set.as_ref().and_then(|s| s.slots.get(v.selected));
    let lines = match (slot, slot.and_then(|s| s.info.as_ref())) {
        (_, Some(i)) => facts::found(i, v),
        (Some(s), None) => facts::missing(s, v),
        (None, None) => Vec::new(),
    };
    let col = div()
        .flex()
        .flex_col()
        .gap_0p5()
        .text_xs()
        .text_color(t.muted);
    col.children(
        v.problem
            .clone()
            .map(|p| div().text_color(t.danger).child(p)),
    )
    .children(lines)
}
