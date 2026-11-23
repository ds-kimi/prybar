//! An item's version history. The Steam API has no call for it, so it
//! is read from the public Workshop changelog page. That page is outside
//! input: parsing never panics, caps every size, and keeps plain text.

mod fetch;
mod retry;
mod text;

pub use fetch::{cached, cached_at, Log};
pub use retry::backoff;

/// Entries past this are dropped: years of daily updates at most.
const MAX_ENTRIES: usize = 500;
const MAX_NOTE: usize = 8_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    /// Unix seconds, when the page gave it.
    pub time: Option<u64>,
    /// The page's own date line, e.g. "Update: 12 Mar, 2023 @ 3:04pm".
    pub headline: String,
    pub note: String,
}

/// Every entry of a changelog page, newest first as Steam lists them.
pub fn parse(html: &str) -> Vec<Change> {
    const HEAD: &str = "changelog headline";
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(at) = rest.find(HEAD) {
        if out.len() == MAX_ENTRIES {
            break;
        }
        rest = &rest[at + HEAD.len()..];
        let headline = text::between(rest, ">", "</div>").unwrap_or_default();
        let next = rest.find(HEAD).unwrap_or(rest.len());
        let body = &rest[..next];
        out.push(Change {
            time: text::stamp(body),
            headline: text::plain(headline, 200),
            note: text::between(body, "<p", "</p>")
                .and_then(|p| p.split_once('>').map(|(_, t)| t))
                .map(|t| text::plain(t, MAX_NOTE))
                .unwrap_or_default(),
        });
    }
    out
}
