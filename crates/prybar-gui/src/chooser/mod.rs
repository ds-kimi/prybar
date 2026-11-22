//! First screen: three cards, one per design, each with a live preview.

mod card;
mod preview;

use gpui::{div, prelude::*, Context, Div};

use crate::actions::MOD;
use crate::app::Root;
use crate::state::DesignKind;
use crate::theme;

pub fn render(cx: &mut Context<Root>) -> Div {
    let palette = theme::chooser();
    let cards = DesignKind::ALL
        .into_iter()
        .enumerate()
        .map(|(ix, kind)| card::card(kind, ix, cx));
    div()
        .size_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_8()
        .child(div().text_2xl().child("Pick a direction"))
        .child(div().text_sm().text_color(palette.muted).child(format!(
            "Click a card or press {MOD}-1 to {MOD}-4. {MOD}-0 comes back here."
        )))
        .child(div().flex().gap_6().children(cards))
}
