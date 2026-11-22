//! First element: proves hover, press and animation work before real views.

#[cfg(test)]
mod tests;

use std::time::Duration;

use gpui::{div, ease_in_out, prelude::*, rgb, Animation, AnimationExt as _, Context, Window};

/// Counts clicks so tests can check that input reaches the view.
#[derive(Default)]
pub struct Hello {
    pub clicks: u32,
}

impl Hello {
    fn button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let label = format!("prybar · clicked {}", self.clicks);
        div()
            .id("hello")
            .debug_selector(|| "hello".into())
            .px_6()
            .py_3()
            .rounded_lg()
            .bg(rgb(0x2b2f3a))
            .text_color(rgb(0xe6e9ef))
            .cursor_pointer()
            .hover(|s| s.bg(rgb(0x3a4050)))
            .active(|s| s.bg(rgb(0x4c5468)))
            .on_click(cx.listener(|this, _, _, cx| {
                this.clicks += 1;
                cx.notify();
            }))
            .child(label)
            // Fade in on every click so motion is visible from the first screen.
            .with_animation(
                ("fade", self.clicks),
                Animation::new(Duration::from_millis(300)).with_easing(ease_in_out),
                |el, t| el.opacity(0.4 + 0.6 * t),
            )
    }
}

impl Render for Hello {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x181a20))
            .child(self.button(cx))
    }
}
