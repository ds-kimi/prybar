//! Every toolbar button: what it shows, what it explains and what it
//! puts in the text. Steam's formatting help is the list.

use gpui::{prelude::*, Context, FontWeight};

use super::super::icon_button;
use super::super::input::{any, Spec};
use super::super::tip::Tipped as _;
use super::BbEditor;
use crate::theme::prybar::Tokens;

/// Tags whose value is asked for in a small prompt first.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ask {
    Link,
    Image,
    Quote,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    /// Text put before and after the selection.
    Wrap(&'static str, &'static str),
    Ask(Ask),
}

/// How a button's glyph is drawn, so B, I, U and S show what they do.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Glyph {
    Plain,
    Bold,
    Italic,
    Underline,
    Strike,
}

pub struct Button {
    pub id: &'static str,
    pub glyph: &'static str,
    pub look: Glyph,
    pub tip: &'static str,
    pub tool: Tool,
}

const fn wrap(id: &'static str, glyph: &'static str, tip: &'static str, tool: Tool) -> Button {
    Button {
        id,
        glyph,
        look: Glyph::Plain,
        tip,
        tool,
    }
}

const fn styled(
    id: &'static str,
    glyph: &'static str,
    look: Glyph,
    tip: &'static str,
    tag: (&'static str, &'static str),
) -> Button {
    Button {
        id,
        glyph,
        look,
        tip,
        tool: Tool::Wrap(tag.0, tag.1),
    }
}

const TABLE_OPEN: &str = "[table]\n[tr]\n[th]Heading[/th]\n[/tr]\n[tr]\n[td]";
const TABLE_CLOSE: &str = "[/td]\n[/tr]\n[/table]";

/// Groups of buttons, drawn with a gap between groups.
pub const TOOLS: [&[Button]; 5] = [
    &[
        wrap("h1", "H1", "Big heading [h1]", Tool::Wrap("[h1]", "[/h1]")),
        wrap("h2", "H2", "Heading [h2]", Tool::Wrap("[h2]", "[/h2]")),
        wrap(
            "h3",
            "H3",
            "Small heading [h3]",
            Tool::Wrap("[h3]", "[/h3]"),
        ),
    ],
    &[
        styled("b", "B", Glyph::Bold, "Bold [b] (mod-b)", ("[b]", "[/b]")),
        styled(
            "i",
            "I",
            Glyph::Italic,
            "Italic [i] (mod-i)",
            ("[i]", "[/i]"),
        ),
        styled(
            "u",
            "U",
            Glyph::Underline,
            "Underline [u] (mod-u)",
            ("[u]", "[/u]"),
        ),
        styled(
            "strike",
            "S",
            Glyph::Strike,
            "Strikethrough [strike]",
            ("[strike]", "[/strike]"),
        ),
        wrap(
            "spoiler",
            "▒",
            "Spoiler, hidden until hovered [spoiler]",
            Tool::Wrap("[spoiler]", "[/spoiler]"),
        ),
    ],
    &[
        wrap("url", "🔗", "Link [url=…] (mod-k)", Tool::Ask(Ask::Link)),
        wrap("img", "🖼", "Image from a link [img]", Tool::Ask(Ask::Image)),
        wrap(
            "quote",
            "❝",
            "Quote, with who said it [quote=…]",
            Tool::Ask(Ask::Quote),
        ),
        wrap(
            "code",
            "</>",
            "Code block, shown as written [code]",
            Tool::Wrap("[code]", "[/code]"),
        ),
        wrap(
            "noparse",
            "[ ]",
            "Show tags as plain text [noparse]",
            Tool::Wrap("[noparse]", "[/noparse]"),
        ),
    ],
    &[
        wrap(
            "list",
            "•",
            "Bulleted list [list]",
            Tool::Wrap("[list]\n[*]", "\n[/list]"),
        ),
        wrap(
            "olist",
            "1.",
            "Numbered list [olist]",
            Tool::Wrap("[olist]\n[*]", "\n[/olist]"),
        ),
        wrap("item", "*", "List item [*]", Tool::Wrap("[*]", "")),
        wrap("hr", "―", "Horizontal rule [hr]", Tool::Wrap("[hr]", "")),
    ],
    &[
        wrap(
            "table",
            "▦",
            "Table with a heading row [table]",
            Tool::Wrap(TABLE_OPEN, TABLE_CLOSE),
        ),
        wrap("tr", "Row", "Table row [tr]", Tool::Wrap("[tr]", "[/tr]")),
        wrap("th", "Th", "Heading cell [th]", Tool::Wrap("[th]", "[/th]")),
        wrap("td", "Td", "Cell [td]", Tool::Wrap("[td]", "[/td]")),
    ],
];

/// The prompt's one-line field, for an address or a name.
pub fn value_spec() -> Spec {
    Spec {
        placeholder: "https://…",
        multiline: false,
        text: String::new(),
        check: any,
        rows: (1, 1),
        markup: false,
    }
}

/// One toolbar button; its glyph shows the style it applies.
pub fn tool(b: &'static Button, t: Tokens, cx: &mut Context<BbEditor>) -> impl IntoElement {
    let id = b.id;
    icon_button(id, b.glyph, t)
        .min_w_7()
        .w_auto()
        .px_1p5()
        .text_sm()
        .debug_selector(move || format!("pb-bb-{id}"))
        .when(b.look == Glyph::Bold, |el| el.font_weight(FontWeight::BOLD))
        .when(b.look == Glyph::Italic, |el| el.italic())
        .when(b.look == Glyph::Underline, |el| el.underline())
        .when(b.look == Glyph::Strike, |el| el.line_through())
        .tip(b.tip, t)
        .on_click(cx.listener(move |this, _, _, cx| this.run(b.tool, cx)))
}
