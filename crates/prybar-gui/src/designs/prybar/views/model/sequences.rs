//! The model's sequences; a click plays one in the viewport, a second
//! click on the playing one goes back to the bind pose.

use gpui::{div, prelude::*, px, AnyElement, Context, SharedString};

use super::super::super::widgets::Tipped as _;
use crate::app::{Forms, Root};
use crate::theme::prybar::Tokens;

/// A model can list thousands; past this the list would only slow the
/// screen down, and the count says what is left.
const SHOWN: usize = 500;

pub fn sequences(f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let vp = f.viewport.read(cx);
    let Some(ir) = vp.ir.clone() else {
        let note = "Sequences appear once the model is loaded.";
        return div().text_color(t.muted).child(note).into_any_element();
    };
    if ir.sequences.is_empty() {
        let note = "This model has no sequences.";
        return div().text_color(t.muted).child(note).into_any_element();
    }
    let playing = vp.play.seq;
    let names = ir.sequences.iter().take(SHOWN).enumerate();
    let rows = names.map(|(i, s)| row(f, i, s.name.clone(), playing == Some(i), t, cx));
    let more = ir.sequences.len().saturating_sub(SHOWN);
    div()
        .id("pb-sequences")
        .flex()
        .flex_col()
        .max_h(px(240.0))
        .overflow_y_scroll()
        .children(rows)
        .children((more > 0).then(|| div().text_color(t.muted).child(format!("{more} more"))))
        .into_any_element()
}

fn row(
    f: &Forms,
    i: usize,
    name: String,
    on: bool,
    t: Tokens,
    cx: &Context<Root>,
) -> impl IntoElement {
    let vp = f.viewport.clone();
    item(i, on, t)
        .tip("Play this sequence in the 3D view; click again to stop", t)
        .on_click(cx.listener(move |_, _, _, cx| {
            vp.update(cx, |v, cx| v.play((!on).then_some(i), cx));
            // The list shows which one plays; the viewport does not
            // repaint the screen around it.
            cx.notify()
        }))
        .child(if name.is_empty() {
            format!("sequence {i}")
        } else {
            name
        })
}

/// The playing row carries a rust edge and full text, not color alone.
fn item(i: usize, on: bool, t: Tokens) -> gpui::Stateful<gpui::Div> {
    let id = SharedString::from(format!("pb-seq-{i}"));
    let selector = id.clone();
    div()
        .id(id)
        .debug_selector(move || selector.to_string())
        .px_2()
        .py_0p5()
        .rounded_md()
        .border_l_2()
        .border_color(if on {
            t.rust
        } else {
            gpui::transparent_black().into()
        })
        .text_color(if on { t.text } else { t.muted })
        .hover(move |s| s.bg(t.hover).text_color(t.text))
        .active(move |s| s.bg(t.active))
}
