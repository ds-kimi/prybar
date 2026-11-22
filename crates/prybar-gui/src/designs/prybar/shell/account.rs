//! The Steam account in the top bar: picture, name and a dot that is
//! green when publishing works and red when it does not. The tooltip says
//! why and what to do; a click checks again.

use gpui::{div, img, prelude::*, px, AnyElement, Context, ObjectFit};

use super::super::widgets::{ring, Tipped as _};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn badge(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let (why, name, dot) = look(s, t);
    let el = div()
        .id("pb-steam")
        .debug_selector(|| "pb-steam".into())
        .h_8()
        .px_1p5()
        .mr_1()
        .flex()
        .items_center()
        .gap_2()
        .rounded_md()
        .cursor_pointer()
        .hover(move |st| st.bg(t.hover))
        .active(move |st| st.bg(t.active))
        .tip(format!("{why} Click to check again."), t)
        .on_click(cx.listener(|this, _, _, cx| this.state.update(cx, |s, cx| s.refresh_steam(cx))))
        .child(face(s, dot, t))
        .child(
            div()
                .max_w(px(140.))
                .truncate()
                .text_color(t.muted)
                .child(name),
        );
    ring(el, t).into_any_element()
}

/// The tooltip sentence, the shown name and the dot color.
fn look(s: &AppState, t: Tokens) -> (String, String, gpui::Rgba) {
    let steam = &s.tools.steam;
    let (why, ok) = steam.sentence();
    let name = match &steam.status {
        Some(prybar_workshop::SteamStatus::Ready { user }) => user.clone(),
        _ => "Steam".to_owned(),
    };
    let dot = match ok {
        Some(true) => t.success,
        Some(false) => t.danger,
        None => t.muted,
    };
    (why, name, dot)
}

/// The picture with the dot on its corner, or a neutral disc until
/// Steam hands one over.
fn face(s: &AppState, dot: gpui::Rgba, t: Tokens) -> impl IntoElement {
    let pic = match s.tools.steam.avatar.clone() {
        Some(image) => img(image)
            .size_6()
            .rounded_full()
            .object_fit(ObjectFit::Cover)
            .into_any_element(),
        None => div()
            .size_6()
            .rounded_full()
            .bg(t.active)
            .into_any_element(),
    };
    div().relative().child(pic).child(
        div()
            .debug_selector(|| "pb-steam-dot".into())
            .absolute()
            .right(px(-2.))
            .bottom(px(-2.))
            .size(px(10.))
            .rounded_full()
            .border_2()
            .border_color(t.surface)
            .bg(dot),
    )
}
