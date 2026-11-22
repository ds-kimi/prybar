//! The command bar's vocabulary: screens, file actions, open files,
//! recent files and designs, filtered by a forgiving fuzzy match.

use std::path::PathBuf;

use crate::state::{AppState, DesignKind, Screen};

#[derive(Clone, Debug, PartialEq)]
pub enum Act {
    Go(Screen),
    Dialog,
    Open(PathBuf),
    Select(usize),
    Decompile,
    Extract,
    SplitQc,
    Design(DesignKind),
}

#[derive(Clone, Debug)]
pub struct Command {
    pub label: String,
    pub hint: &'static str,
    pub act: Act,
}

/// Rows the bar shows; the query narrows the rest.
pub const SHOWN: usize = 12;

/// Commands matching the bar's query, best first, at most `SHOWN`. A query that looks
/// like a path is offered as a file to open first.
pub fn matches(s: &AppState) -> Vec<Command> {
    let raw = s.search.as_deref().unwrap_or("").trim();
    let q = raw.to_lowercase();
    let mut out: Vec<Command> = Vec::new();
    if raw.starts_with('/') || raw.starts_with('~') || raw.contains(":\\") {
        out.push(Command {
            label: format!("open {raw}"),
            hint: "path",
            act: Act::Open(PathBuf::from(raw)),
        });
    }
    let mut hits: Vec<(u8, Command)> = all(s)
        .into_iter()
        .filter_map(|c| score(&c.label.to_lowercase(), &q).map(|r| (r, c)))
        .collect();
    hits.sort_by_key(|(r, _)| *r);
    out.extend(hits.into_iter().map(|(_, c)| c));
    out.truncate(SHOWN);
    out
}

fn all(s: &AppState) -> Vec<Command> {
    let cmd = |label: String, hint: &'static str, act: Act| Command { label, hint, act };
    let mut out: Vec<Command> = Screen::ALL
        .iter()
        .map(|sc| {
            cmd(
                format!("go {}", sc.label().to_lowercase()),
                "screen",
                Act::Go(*sc),
            )
        })
        .collect();
    out.push(cmd("open file…".into(), "dialog", Act::Dialog));
    out.push(cmd("decompile".into(), "run", Act::Decompile));
    out.push(cmd("extract all".into(), "run", Act::Extract));
    out.push(cmd("toggle split-qc".into(), "option", Act::SplitQc));
    let docs = s.documents.iter().enumerate();
    out.extend(docs.map(|(i, d)| cmd(format!("file {}", d.title()), "file", Act::Select(i))));
    let recent = s.recent.iter();
    out.extend(recent.map(|p| {
        cmd(
            format!("recent {}", p.display()),
            "path",
            Act::Open(p.clone()),
        )
    }));
    out.extend(designs());
    out
}

fn designs() -> impl Iterator<Item = Command> {
    DesignKind::ALL.into_iter().map(|d| Command {
        label: format!("design {}", d.name().to_lowercase()),
        hint: "design",
        act: Act::Design(d),
    })
}

/// `None` unless every typed letter appears in order (spaces ignored,
/// so "gs" finds "go settings"). Lower is better: whole-word hits first,
/// so "decompile" runs decompile rather than "go decompile".
fn score(label: &str, q: &str) -> Option<u8> {
    let mut it = label.chars();
    let fits = q
        .chars()
        .filter(|c| !c.is_whitespace())
        .all(|n| it.any(|h| h == n));
    match () {
        _ if !fits => None,
        _ if q.is_empty() || label == q || label.starts_with(q) => Some(0),
        _ if label.contains(q) => Some(1),
        _ => Some(2),
    }
}
