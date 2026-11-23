//! BBCode nodes drawn close to the Workshop page: inline runs become
//! wrapped paragraphs, the rest are blocks laid out top to bottom.

mod image;
mod inline;
mod parts;
mod table;
mod text;

use std::cell::Cell;
use std::collections::HashMap;

use gpui::{div, prelude::*, AnyElement, Div, ElementId, FontWeight};
use prybar_workshop::bbcode::{Node, Tag};

use super::Shot;
use crate::theme::prybar::Tokens;
use inline::Para;
pub use text::text_of;

/// What drawing needs besides the nodes: colors, images fetched so far,
/// and a counter so every interactive part gets its own id.
pub struct Look<'a> {
    pub t: Tokens,
    pub shots: &'a HashMap<String, Shot>,
    ids: Cell<usize>,
}

impl<'a> Look<'a> {
    pub fn new(t: Tokens, shots: &'a HashMap<String, Shot>) -> Self {
        Look {
            t,
            shots,
            ids: Cell::new(0),
        }
    }

    fn id(&self, what: &'static str) -> ElementId {
        let n = self.ids.get();
        self.ids.set(n + 1);
        (what, n).into()
    }
}

/// Nodes as a column: runs of inline nodes join into one paragraph.
pub fn blocks(nodes: &[Node], l: &Look) -> Div {
    let mut out = div().flex().flex_col().gap_2().min_w_0();
    let mut para = Para::default();
    for n in nodes {
        if text::is_inline(n) {
            para.node(n, Default::default(), l.t);
            continue;
        }
        out = out.children(para.take(l)).child(block(n, l));
    }
    out.children(para.take(l))
}

fn block(n: &Node, l: &Look) -> AnyElement {
    let t = l.t;
    let Node::Tag(tag, value, kids) = n else {
        return div().h_px().my_1().bg(t.border).into_any_element();
    };
    let heading = |size: f32| {
        let el = div()
            .text_size(gpui::px(size))
            .font_weight(FontWeight::SEMIBOLD);
        el.child(blocks(kids, l)).into_any_element()
    };
    match tag {
        Tag::H1 => heading(22.),
        Tag::H2 => heading(18.),
        Tag::H3 => heading(15.),
        Tag::List | Tag::Olist => parts::list(kids, *tag == Tag::Olist, l),
        Tag::Quote => parts::quote(value.as_deref(), kids, l),
        Tag::Code => parts::code(kids, l),
        Tag::Spoiler => parts::spoiler(kids, l),
        Tag::Table => table::table(value.as_deref(), kids, l),
        Tag::Img => image::image(&text_of(kids), l),
        Tag::Url => image::link(value.clone().unwrap_or_else(|| text_of(kids)), kids, l),
        _ => blocks(kids, l).into_any_element(),
    }
}
