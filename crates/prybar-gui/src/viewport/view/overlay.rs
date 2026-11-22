//! The quiet strip of view toggles in the corner of the viewport. Each
//! names its key so the keyboard path is discoverable.

use gpui::{div, prelude::*, Context, SharedString};

use super::{Full, Viewport};
use crate::theme::prybar::Tokens;

type Act = fn(&mut Viewport, &mut Context<Viewport>);

pub fn controls(v: &Viewport, t: &Tokens, cx: &mut Context<Viewport>) -> impl IntoElement {
    let f = v.flags;
    let chips: [(&'static str, &'static str, bool, Act); 6] = [
        ("pb-vp-wire", "Wire  W", f.wire, |v, _| {
            v.flags.wire = !v.flags.wire
        }),
        ("pb-vp-bones", "Bones  B", f.bones, |v, _| {
            v.flags.bones = !v.flags.bones
        }),
        ("pb-vp-normals", "Normals", f.normals, |v, _| {
            v.flags.normals = !v.flags.normals
        }),
        ("pb-vp-backdrop", "Grey", v.backdrop, |v, _| {
            v.backdrop = !v.backdrop
        }),
        ("pb-vp-frame", "Frame  F", false, |v, cx| {
            v.reframe(false, cx)
        }),
        ("pb-vp-full", "Full size  ⇧F", false, |_, cx| {
            cx.emit(Full)
        }),
    ];
    let row = div().absolute().top_2().right_2().flex().gap_1();
    row.children(chips.map(|(id, label, on, act)| chip(id, label, on, t, cx, act)))
}

/// A toggle with hover, press and on states; clicks never reach the
/// viewport so they do not start a drag.
fn chip(
    id: &'static str,
    label: &'static str,
    on: bool,
    t: &Tokens,
    cx: &mut Context<Viewport>,
    act: impl Fn(&mut Viewport, &mut Context<Viewport>) + 'static,
) -> impl IntoElement {
    let (hover, active) = (t.hover, t.active);
    div()
        .id(id)
        .debug_selector(move || id.into())
        .px_2()
        .py_0p5()
        .rounded_md()
        .text_xs()
        .border_1()
        .border_color(if on { t.rust } else { t.border })
        .text_color(if on { t.text } else { t.muted })
        .bg(t.surface)
        .hover(move |s| s.bg(hover))
        .active(move |s| s.bg(active))
        .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_click(cx.listener(move |v, _, _, cx| {
            act(v, cx);
            cx.notify()
        }))
        .child(SharedString::from(label))
}
