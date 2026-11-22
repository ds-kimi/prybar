//! What Garry's Mod thinks of the chosen addon folder: its type and
//! tags, and every file it would refuse with the reason.

use gpui::{div, prelude::*, AnyElement};

use prybar_workshop::addon::AddonCheck;

use crate::state::AppState;
use crate::theme::prybar::Tokens;

/// Long lists stay readable; the count says how many more there are.
const SHOWN: usize = 12;

pub fn view(s: &AppState, t: Tokens) -> AnyElement {
    let Some((_, found)) = &s.tools.steam.addon else {
        return div().into_any_element();
    };
    let body = div()
        .debug_selector(|| "pb-pub-addon".into())
        .flex()
        .flex_col()
        .gap_1();
    let check = match found {
        Err(why) => {
            return body
                .text_color(t.danger)
                .child(why.clone())
                .into_any_element()
        }
        Ok(check) => check,
    };
    body.child(
        div()
            .text_color(if check.ok() { t.muted } else { t.text })
            .child(summary(check)),
    )
    .children(
        problems(check)
            .into_iter()
            .map(|line| div().text_color(t.danger).child(line)),
    )
    .into_any_element()
}

fn summary(check: &AddonCheck) -> String {
    let tags = if check.tags.is_empty() {
        "none".to_owned()
    } else {
        check.tags.join(", ")
    };
    format!(
        "{}: {} · tags {tags} · {} files to pack",
        check.title, check.kind, check.files
    )
}

/// `addon.json` problems first: fixing one may change the file list.
/// Past [`SHOWN`] lines, one more line counts the rest.
fn problems(check: &AddonCheck) -> Vec<String> {
    let json = check.problems.iter().map(|p| format!("addon.json: {p}"));
    let files = check
        .rejected
        .iter()
        .map(|r| format!("{}: {}", r.path, r.reason));
    let mut all: Vec<String> = json.chain(files).collect();
    let more = all.len().saturating_sub(SHOWN);
    all.truncate(SHOWN);
    if more > 0 {
        all.push(format!("and {more} more"));
    }
    all
}
