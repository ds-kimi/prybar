//! A paragraph: text with bold, italic, underline, strike and link runs,
//! drawn as one wrapped text so lines break like on the Workshop.

use std::ops::Range;

use gpui::{
    div, prelude::*, px, AnyElement, FontStyle, FontWeight, HighlightStyle, InteractiveText,
    StrikethroughStyle, StyledText, UnderlineStyle,
};
use prybar_workshop::bbcode::{Node, Tag};

use super::super::super::tip::card;
use super::{text_of, Look};
use crate::theme::prybar::Tokens;

#[derive(Default)]
pub struct Para {
    text: String,
    runs: Vec<(Range<usize>, HighlightStyle)>,
    links: Vec<(Range<usize>, String)>,
}

impl Para {
    /// Adds an inline node in `style`, the styles of the tags around it.
    pub fn node(&mut self, n: &Node, style: HighlightStyle, t: Tokens) {
        let (tag, value, kids) = match n {
            Node::Text(s) => return self.push(s, style),
            Node::Tag(tag, value, kids) => (*tag, value, kids),
            Node::Rule => return,
        };
        let style = styled(tag, style, t);
        let start = self.text.len();
        if tag == Tag::Noparse {
            self.push(&text_of(kids), style);
        } else {
            kids.iter().for_each(|k| self.node(k, style, t));
        }
        if tag == Tag::Url {
            let href = value
                .clone()
                .unwrap_or_else(|| text_of(kids))
                .trim()
                .to_owned();
            self.links.push((start..self.text.len(), href));
        }
    }

    /// Line breaks right after a block belong to it, not to this text.
    fn push(&mut self, s: &str, style: HighlightStyle) {
        let s = if self.text.is_empty() {
            s.trim_start_matches(['\n', '\r'])
        } else {
            s
        };
        let start = self.text.len();
        self.text.push_str(s);
        if style != HighlightStyle::default() && !s.is_empty() {
            self.runs.push((start..self.text.len(), style));
        }
    }

    /// The paragraph so far as one element, and a fresh one to go on;
    /// `None` when it holds only blank space.
    pub fn take(&mut self, l: &Look) -> Option<AnyElement> {
        let mut p = std::mem::take(self);
        let end = p.text.trim_end().len();
        if end == 0 {
            return None;
        }
        p.text.truncate(end);
        let clip = |r: &Range<usize>| r.start.min(end)..r.end.min(end);
        let runs: Vec<_> = p.runs.iter().map(|(r, s)| (clip(r), *s)).collect();
        let styled = StyledText::new(p.text).with_highlights(runs);
        if p.links.is_empty() {
            return Some(div().w_full().child(styled).into_any_element());
        }
        let (ranges, hrefs): (Vec<_>, Vec<_>) =
            p.links.into_iter().map(|(r, h)| (clip(&r), h)).unzip();
        let (t, tips) = (l.t, hrefs.clone());
        let text = InteractiveText::new(l.id("pb-bb-text"), styled)
            .on_click(ranges, move |i, _, cx| {
                if let Some(h) = hrefs.get(i).filter(|h| h.starts_with("http")) {
                    cx.open_url(h);
                }
            })
            .tooltip(move |i, _, cx| Some(card(tips.get(i)?.clone(), t, cx)));
        Some(div().w_full().child(text).into_any_element())
    }
}

/// The style inside `tag`, on top of the styles around it.
fn styled(tag: Tag, style: HighlightStyle, t: Tokens) -> HighlightStyle {
    let line = Some(UnderlineStyle {
        thickness: px(1.),
        ..Default::default()
    });
    let strike = Some(StrikethroughStyle {
        thickness: px(1.),
        color: None,
    });
    let mut s = style;
    match tag {
        Tag::Bold => s.font_weight = Some(FontWeight::BOLD),
        Tag::Italic => s.font_style = Some(FontStyle::Italic),
        Tag::Underline => s.underline = line,
        Tag::Strike => s.strikethrough = strike,
        Tag::Url => {
            s.color = Some(t.rust.into());
            s.underline = line;
        }
        _ => {}
    }
    s
}
