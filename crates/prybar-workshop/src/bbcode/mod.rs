//! Steam Workshop description markup (BBCode) read into a tree the GUI
//! can draw. Descriptions come from other people, so nothing here may
//! panic: unknown or broken tags simply stay as the text they were.

mod item;
mod lex;
mod parse;
#[cfg(test)]
mod tests;
mod text;
mod tree;

pub use parse::parse;
pub use text::plain;

/// Every tag Steam's formatting help lists for Workshop descriptions.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tag {
    H1,
    H2,
    H3,
    Bold,
    Italic,
    Underline,
    Strike,
    Spoiler,
    Noparse,
    Rule,
    Url,
    Img,
    List,
    Olist,
    Item,
    Quote,
    Code,
    Table,
    Tr,
    Th,
    Td,
}

/// What a description reads as. Text keeps its line breaks.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Node {
    Text(String),
    /// `[hr]`, which has no closing tag.
    Rule,
    /// A tag, its value (`[url=…]`, `[quote=…]`, table options) and
    /// what it holds.
    Tag(Tag, Option<String>, Vec<Node>),
}

/// Name, tag pairs; names are what is written between the brackets.
const NAMES: [(&str, Tag); 21] = [
    ("h1", Tag::H1),
    ("h2", Tag::H2),
    ("h3", Tag::H3),
    ("b", Tag::Bold),
    ("i", Tag::Italic),
    ("u", Tag::Underline),
    ("strike", Tag::Strike),
    ("spoiler", Tag::Spoiler),
    ("noparse", Tag::Noparse),
    ("hr", Tag::Rule),
    ("url", Tag::Url),
    ("img", Tag::Img),
    ("list", Tag::List),
    ("olist", Tag::Olist),
    ("*", Tag::Item),
    ("quote", Tag::Quote),
    ("code", Tag::Code),
    ("table", Tag::Table),
    ("tr", Tag::Tr),
    ("th", Tag::Th),
    ("td", Tag::Td),
];

impl Tag {
    /// The tag written `name`, any case; `None` for tags Steam lacks.
    pub fn named(name: &str) -> Option<Tag> {
        NAMES
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, t)| *t)
    }

    /// The name written between the brackets.
    pub fn name(self) -> &'static str {
        NAMES
            .iter()
            .find(|(_, t)| *t == self)
            .map_or("", |(n, _)| n)
    }
}
