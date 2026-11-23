//! The editor as drawn: toolbar, optional prompt, then the field or the
//! preview, switched by an Edit / Preview pair at the toolbar's end.

use gpui::{div, prelude::*, AnyElement, Context, Div, Focusable as _, Stateful, Window};
use prybar_workshop::bbcode::parse;

use super::super::icon_button;
use super::super::tip::Tipped as _;
use super::render::{blocks, Look};
use super::tools::{self, TOOLS};
use super::BbEditor;
use crate::theme::prybar::{tokens, Light, Tokens};

impl Render for BbEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(cx.try_global::<Light>().is_some_and(|l| l.0));
        if std::mem::take(&mut self.focus_value) {
            self.value.read(cx).focus_handle(cx).focus(window, cx);
        } else if std::mem::take(&mut self.focus_input) {
            self.input.read(cx).focus_handle(cx).focus(window, cx);
        }
        let body = match self.preview {
            true => self.preview_body(t, cx),
            false => self.input.clone().into_any_element(),
        };
        div()
            .flex()
            .flex_col()
            .gap_1p5()
            .child(self.toolbar(t, cx))
            .children(self.asking.map(|a| self.prompt(a, t, cx)))
            .child(body)
    }
}

impl BbEditor {
    /// Buttons wrap to more rows on a narrow popup; in preview they are
    /// hidden, since there is no selection to format.
    fn toolbar(&self, t: Tokens, cx: &mut Context<Self>) -> impl IntoElement {
        let mut bar = div()
            .debug_selector(|| "pb-bb-toolbar".into())
            .flex()
            .flex_wrap()
            .items_center()
            .gap_x_3()
            .gap_y_1();
        if !self.preview {
            for group in TOOLS {
                let buttons = group.iter().map(|b| tools::tool(b, t, cx));
                bar = bar.child(div().flex().gap_0p5().children(buttons));
            }
        }
        let edit = mode(("pb-bb-edit", "Edit"), self.preview, false, t, cx);
        let edit = edit.tip("Write the description", t);
        let show = mode(("pb-bb-preview", "Preview"), self.preview, true, t, cx);
        let show = show.tip("See it as the Workshop shows it", t);
        bar.child(div().ml_auto().flex().gap_0p5().child(edit).child(show))
    }

    /// The description drawn as Steam would, in a box the field's size.
    fn preview_body(&mut self, t: Tokens, cx: &mut Context<Self>) -> AnyElement {
        let nodes = parse(self.input.read(cx).text());
        self.want(&nodes, cx);
        let look = Look::new(t, self.shots());
        div()
            .id("pb-bb-preview-body")
            .debug_selector(|| "pb-bb-preview-body".into())
            .min_h_40()
            .max_h_96()
            .overflow_y_scroll()
            .p_3()
            .rounded_md()
            .border_1()
            .border_color(t.border)
            .bg(t.bg)
            .text_sm()
            .child(blocks(&nodes, &look))
            .into_any_element()
    }
}

/// Half of the Edit / Preview pair; the current one stays tinted.
fn mode(
    (id, label): (&'static str, &'static str),
    now: bool,
    on: bool,
    t: Tokens,
    cx: &mut Context<BbEditor>,
) -> Stateful<Div> {
    icon_button(id, label, t)
        .w_auto()
        .px_2()
        .debug_selector(move || id.into())
        .when(now == on, move |el| el.bg(t.active).text_color(t.text))
        .on_click(cx.listener(move |this, _, _, cx| {
            this.preview = on;
            this.focus_input = !on;
            cx.notify()
        }))
}
