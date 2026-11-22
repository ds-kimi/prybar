//! The two bar ends that are pictures rather than words: the logo,
//! which goes Home, and the theme toggle icon.

use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

use gpui::{
    div, ease_out_quint, img, prelude::*, radians, svg, Animation, AnimationExt as _, AnyElement,
    Context, FontWeight, SharedString, Transformation, Window,
};

use super::super::widgets::ring;
use crate::actions::ToggleTheme;
use crate::app::Root;
use crate::state::Screen;
use crate::theme::prybar::Tokens;

/// 24 px points in a 48 px bar; the 256 px source stays crisp at 2x.
pub fn logo(t: Tokens, cx: &Context<Root>) -> AnyElement {
    let el = div()
        .id("pb-logo")
        .debug_selector(|| "pb-logo".into())
        .aria_label("Home")
        .size_8()
        .mr_2()
        .flex()
        .items_center()
        .justify_center()
        .rounded_md()
        .cursor_pointer()
        .hover(move |s| s.bg(t.hover))
        .active(move |s| s.bg(t.active))
        .child(img("prybar.png").size_6())
        .on_click(cx.listener(|this, _, _, cx| {
            this.state.update(cx, |s, cx| s.navigate(Screen::Home, cx))
        }));
    ring(el, t).into_any_element()
}

/// Sun while dark, moon while light: the icon shows where a click goes.
/// Keyed by theme so each switch replays the turn-in.
pub fn theme_toggle(light: bool, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let (icon, tip) = if light {
        ("icons/moon.svg", "Dark theme")
    } else {
        ("icons/sun.svg", "Light theme")
    };
    let el = div()
        .id("pb-theme")
        .debug_selector(|| "pb-theme".into())
        .aria_label(tip)
        .size_8()
        .flex()
        .items_center()
        .justify_center()
        .rounded_md()
        .cursor_pointer()
        .hover(move |s| s.bg(t.hover))
        .active(move |s| s.bg(t.active))
        .tooltip(move |_, cx| cx.new(|_| Tip(tip.into(), t)).into())
        .on_click(cx.listener(|_, _, w, cx| w.dispatch_action(Box::new(ToggleTheme), cx)))
        .child(glyph(icon, light, t));
    ring(el, t).into_any_element()
}

/// A quarter turn in while fading up, so the swap reads as one motion.
fn glyph(icon: &'static str, light: bool, t: Tokens) -> AnyElement {
    svg()
        .path(icon)
        .size_4()
        .text_color(t.text)
        .with_animation(
            SharedString::from(format!("pb-theme-{light}")),
            Animation::new(Duration::from_millis(220)).with_easing(ease_out_quint()),
            |el, p| {
                el.opacity(p)
                    .with_transformation(Transformation::rotate(radians(-FRAC_PI_2 * (1. - p))))
            },
        )
        .into_any_element()
}

/// The tooltip doubles as the button's name, since the icon has no text.
pub(super) struct Tip(pub SharedString, pub Tokens);

impl Render for Tip {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let t = self.1;
        // Release notes also land here, so long text wraps instead of
        // stretching across the window.
        div()
            .max_w(gpui::px(360.))
            .px_2()
            .py_1()
            .rounded_md()
            .bg(t.surface)
            .border_1()
            .border_color(t.border)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .text_color(t.text)
            .child(self.0.clone())
    }
}
