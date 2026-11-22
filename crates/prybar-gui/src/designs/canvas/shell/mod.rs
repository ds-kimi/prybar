//! The frame: a top bar with the logo and screen tabs, the strip of
//! open file cards, the step path and floating toasts.

pub mod steps;
pub mod toasts;

use gpui::{div, prelude::*, AnyElement, Context};

use super::{motion, widgets};
use crate::app::Root;
use crate::state::{AppState, Document, Screen};
use crate::theme::Palette;

/// Logo on the left, every screen as a rounded tab on the right.
pub fn top(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let tabs = Screen::ALL
        .into_iter()
        .map(|screen| tab(screen, screen == s.screen, p, cx));
    let logo = div()
        .flex()
        .items_center()
        .gap_2()
        .child(div().size_6().rounded_lg().bg(p.accent))
        .child(div().text_lg().child("prybar"));
    div()
        .id("cv-top")
        .debug_selector(|| "cv-top".into())
        .flex_none()
        .px_8()
        .py_4()
        .flex()
        .items_center()
        .gap_6()
        .child(logo)
        .child(div().flex().flex_wrap().gap_1().children(tabs))
        .into_any_element()
}

fn tab(screen: Screen, current: bool, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let label = screen.label();
    div()
        .id(label)
        .debug_selector(move || format!("nav-{}", label.to_lowercase()))
        .px_3()
        .py_1p5()
        .rounded_full()
        .text_sm()
        .cursor_pointer()
        .tab_index(0)
        .border_1()
        .border_color(gpui::transparent_black())
        .text_color(if current { p.surface } else { p.muted })
        .when(current, |el| el.bg(p.accent).shadow_sm())
        .when(!current, |el| {
            el.hover(move |s| s.bg(p.hover).text_color(p.text))
                .active(move |s| s.bg(p.active))
        })
        .focus_visible(move |s| s.border_color(p.accent))
        .on_click(
            cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| s.navigate(screen, cx))
            }),
        )
        .child(label)
}

/// Open files as small cards with a colored thumbnail; click selects.
pub fn files(s: &AppState, p: Palette, cx: &Context<Root>) -> AnyElement {
    let cards = s
        .documents
        .iter()
        .enumerate()
        .map(|(ix, d)| file(ix, d, s.selected == Some(ix), p, cx));
    div()
        .flex()
        .flex_wrap()
        .gap_3()
        .children(cards)
        .into_any_element()
}

fn file(ix: usize, d: &Document, on: bool, p: Palette, cx: &Context<Root>) -> AnyElement {
    let (kind, art) = match d {
        Document::Model { .. } => ("Model", p.accent),
        Document::Package { .. } => ("Package", p.muted),
    };
    let fade = if on { 1. } else { 0.5 };
    let thumb = div().size_8().rounded_lg().bg(art).opacity(fade);
    let card = widgets::card(("cv-file", ix), p)
        .debug_selector(move || format!("cv-file-{ix}"))
        .p_2()
        .pr_4()
        .flex()
        .items_center()
        .gap_3()
        .when(on, |el| el.border_2().border_color(p.accent))
        .on_click(
            cx.listener(move |this, _, _, cx| this.state.update(cx, |s, cx| s.select(ix, cx))),
        )
        .child(thumb)
        .child(
            div()
                .flex()
                .flex_col()
                .child(div().text_sm().child(d.title()))
                .child(div().text_xs().text_color(p.muted).child(kind)),
        );
    motion::enter(div().child(card), ("cv-file-in", ix), ix)
}
