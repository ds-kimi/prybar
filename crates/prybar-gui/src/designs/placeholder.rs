//! Later-work regions and the stub's action button.

use std::time::Duration;

use gpui::{div, prelude::*, pulsating_between, Animation, AnimationExt as _, AnyElement, Context};

use crate::app::Root;
use crate::state::{Placeholder, Screen};
use crate::theme::Palette;

/// A breathing dashed region: clearly not built yet, never a dead box.
pub fn region(ph: Placeholder, p: Palette) -> AnyElement {
    div()
        .id("placeholder")
        .debug_selector(|| "placeholder".into())
        .flex_1()
        .min_h_64()
        .flex()
        .items_center()
        .justify_center()
        .rounded_xl()
        .border_2()
        .border_dashed()
        .border_color(p.border)
        .text_color(p.muted)
        .child(ph.label())
        .with_animation(
            "placeholder-breath",
            Animation::new(Duration::from_millis(2200))
                .repeat()
                .with_easing(pulsating_between(0.5, 1.)),
            |el, t| el.opacity(t),
        )
        .into_any_element()
}

/// Decompile on the decompile screen, extract on the package screen;
/// output goes to a temp folder until the design lots add a picker.
pub fn button(screen: Screen, p: Palette, cx: &mut Context<Root>) -> AnyElement {
    let (label, extract) = match screen {
        Screen::Package => ("Extract all", true),
        _ => ("Decompile", false),
    };
    let out = std::env::temp_dir().join("prybar-gui-out");
    div()
        .id("run")
        .debug_selector(|| "run".into())
        .mt_3()
        .px_4()
        .py_2()
        .rounded_md()
        .bg(p.accent)
        .text_color(p.bg)
        .cursor_pointer()
        .hover(|s| s.opacity(0.9))
        .active(|s| s.opacity(0.75))
        .on_click(cx.listener(move |this, _, _, cx| {
            let out = out.clone();
            this.state.update(cx, |s, cx| match extract {
                true => s.extract_selected(out.join("extract"), cx),
                false => s.decompile_selected(out.join("decompile"), cx),
            })
        }))
        .child(label)
        .into_any_element()
}
