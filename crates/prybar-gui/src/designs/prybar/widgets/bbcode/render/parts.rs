//! Lists, quotes, code blocks and spoilers.

use gpui::{div, prelude::*, AnyElement};
use prybar_workshop::bbcode::{Node, Tag};

use super::super::super::tip::Tipped as _;
use super::{blocks, text_of, Look};

/// Items with a bullet or their number; blank text between them is the
/// line breaks around `[*]` and is dropped.
pub fn list(kids: &[Node], ordered: bool, l: &Look) -> AnyElement {
    let mut n = 0;
    let rows = kids.iter().filter_map(|k| {
        let Node::Tag(Tag::Item, _, inner) = k else {
            let blank = matches!(k, Node::Text(s) if s.trim().is_empty());
            return (!blank).then(|| blocks(std::slice::from_ref(k), l).into_any_element());
        };
        n += 1;
        let mark = if ordered {
            format!("{n}.")
        } else {
            "•".into()
        };
        let mark = div().w_5().flex_none().text_color(l.t.muted).child(mark);
        let row = div()
            .flex()
            .gap_1()
            .child(mark)
            .child(blocks(inner, l).flex_1());
        Some(row.into_any_element())
    });
    div()
        .flex()
        .flex_col()
        .gap_1()
        .pl_1()
        .children(rows.collect::<Vec<_>>())
        .into_any_element()
}

/// A bar on the left, and who said it when the tag names them.
pub fn quote(author: Option<&str>, kids: &[Node], l: &Look) -> AnyElement {
    let t = l.t;
    let by = author.map(|a| {
        let line = format!("Originally posted by {a}:");
        div().text_xs().text_color(t.muted).child(line)
    });
    div()
        .flex()
        .flex_col()
        .gap_1()
        .pl_3()
        .py_1p5()
        .pr_2()
        .border_l_2()
        .border_color(t.rust)
        .bg(t.hover)
        .rounded_r_md()
        .children(by)
        .child(blocks(kids, l))
        .into_any_element()
}

/// Monospace, lines as written, scrolling sideways instead of wrapping.
pub fn code(kids: &[Node], l: &Look) -> AnyElement {
    let t = l.t;
    let text = text_of(kids);
    let text = text.trim_start_matches(['\r', '\n']).trim_end();
    let lines = text.lines().map(|s| {
        let s = if s.is_empty() { " " } else { s };
        div().whitespace_nowrap().child(s.to_owned())
    });
    div()
        .id(l.id("pb-bb-code"))
        .overflow_x_scroll()
        .p_2()
        .rounded_md()
        .bg(t.active)
        .font_family("Menlo")
        .text_xs()
        .child(div().flex().flex_col().children(lines.collect::<Vec<_>>()))
        .into_any_element()
}

/// A covered block until hovered, like the Workshop's black bar.
pub fn spoiler(kids: &[Node], l: &Look) -> AnyElement {
    let t = l.t;
    div()
        .id(l.id("pb-bb-spoiler"))
        .debug_selector(|| "pb-bb-spoiler".into())
        .px_1p5()
        .py_0p5()
        .rounded_sm()
        .bg(t.muted)
        .text_color(t.muted)
        .hover(move |s| s.bg(t.active).text_color(t.text))
        .tip("Spoiler: hover to read", t)
        .child(blocks(kids, l))
        .into_any_element()
}
