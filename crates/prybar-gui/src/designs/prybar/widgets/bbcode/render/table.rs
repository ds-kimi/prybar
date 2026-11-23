//! Tables: rows of equal cells, heading cells bold on a tint. Steam's
//! `noborder=1` drops the lines.

use gpui::{div, prelude::*, AnyElement, Div, FontWeight};
use prybar_workshop::bbcode::{Node, Tag};

use super::{blocks, Look};

pub fn table(options: Option<&str>, kids: &[Node], l: &Look) -> AnyElement {
    let lines = !options.is_some_and(|o| o.contains("noborder=1"));
    let rows = kids.iter().filter_map(|k| match k {
        Node::Tag(Tag::Tr, _, cells) => Some(row(cells, lines, l)),
        _ => None,
    });
    div()
        .flex()
        .flex_col()
        .rounded_md()
        .overflow_hidden()
        .when(lines, |el| el.border_1().border_color(l.t.border))
        .children(rows.collect::<Vec<_>>())
        .into_any_element()
}

fn row(cells: &[Node], lines: bool, l: &Look) -> Div {
    let cells = cells.iter().filter_map(|c| match c {
        Node::Tag(tag @ (Tag::Th | Tag::Td), _, kids) => {
            Some(cell(*tag == Tag::Th, kids, lines, l))
        }
        _ => None,
    });
    div()
        .flex()
        .when(lines, |el| el.border_b_1().border_color(l.t.border))
        .children(cells.collect::<Vec<_>>())
}

fn cell(head: bool, kids: &[Node], lines: bool, l: &Look) -> Div {
    div()
        .flex_1()
        .min_w_0()
        .px_2()
        .py_1()
        .when(lines, |el| el.border_r_1().border_color(l.t.border))
        .when(head, |el| {
            el.font_weight(FontWeight::SEMIBOLD).bg(l.t.hover)
        })
        .child(blocks(kids, l))
}
