//! Package: a search bubble, then one card per folder with its files.

use std::collections::BTreeMap;

use gpui::{div, prelude::*, AnyElement, Context};
use prybar_service::ops::ListReport;

use super::super::{motion, widgets};
use crate::actions::{Search, MOD};
use crate::app::Root;
use crate::theme::Palette;

/// Files drawn before "and N more"; keeps huge packages responsive.
const MAX_FILES: usize = 300;

pub fn view(
    report: &ListReport,
    search: Option<&str>,
    p: Palette,
    cx: &Context<Root>,
) -> AnyElement {
    let query = search.unwrap_or("").to_ascii_lowercase();
    let (folders, count) = group(report, &query);
    let more = count.saturating_sub(MAX_FILES);
    let text = match search {
        Some(q) => format!("🔍 {q}▏   {count} found"),
        None => format!("🔍 Find a file…  ({MOD}-f)"),
    };
    let bar = widgets::search("cv-search", text, search.is_some(), p)
        .on_click(|_, window, cx| window.dispatch_action(Box::new(Search), cx));
    let cards = folders
        .into_iter()
        .enumerate()
        .map(|(ix, (d, f))| folder(ix, d, f, p));
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(header(report, p, cx))
        .child(bar)
        .child(div().flex().flex_wrap().gap_4().children(cards))
        .children((more > 0).then(|| div().text_color(p.muted).child(format!("and {more} more"))))
        .into_any_element()
}

fn header(report: &ListReport, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let out = std::env::temp_dir().join("prybar-gui-out").join("extract");
    let extract = widgets::pill("cv-extract", "Extract everything", true, true, p)
        .debug_selector(|| "cv-extract".into())
        .on_click(cx.listener(move |this, _, _, cx| {
            let out = out.clone();
            this.state.update(cx, |s, cx| s.extract_selected(out, cx))
        }));
    let hint = format!(
        "{:?} · {} files · {} bytes",
        report.format, report.total, report.total_bytes
    );
    div()
        .flex()
        .items_center()
        .gap_4()
        .child(div().flex_1().child(widgets::title("Package", hint, p)))
        .child(extract)
}

fn folder(ix: usize, dir: &str, files: Vec<&str>, p: Palette) -> AnyElement {
    let rows = files
        .into_iter()
        .map(|f| div().text_sm().child(f.to_owned()));
    let name = if dir.is_empty() {
        "(root)".to_owned()
    } else {
        format!("{dir}/")
    };
    let card = widgets::card(("cv-folder", ix), p)
        .w_72()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_color(p.accent).child(format!("📁 {name}")))
        .children(rows);
    motion::enter(div().child(card), ("cv-folder-in", ix), ix)
}

/// Matching files by folder, capped at `MAX_FILES`, plus the full count.
fn group<'a>(report: &'a ListReport, query: &str) -> (BTreeMap<&'a str, Vec<&'a str>>, usize) {
    let mut folders: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let hits = report
        .entries
        .iter()
        .filter(|e| e.path.to_ascii_lowercase().contains(query));
    let mut count = 0;
    for e in hits {
        count += 1;
        if count <= MAX_FILES {
            let (dir, name) = e.path.rsplit_once('/').unwrap_or(("", &e.path));
            folders.entry(dir).or_default().push(name);
        }
    }
    (folders, count)
}
