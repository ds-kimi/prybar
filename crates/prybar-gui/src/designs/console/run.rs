//! Runs what the command bar picked. Called by the root view's key
//! handlers, which own the window (the open dialog needs it).

use gpui::{Context, Window};

use super::commands::{self, Act};
use super::views::out_dir;
use crate::actions::Open;
use crate::app::Root;
use crate::state::{AppState, Document, Screen};

/// Enter: runs the highlighted command, if the query matches any.
pub fn submit(root: &mut Root, window: &mut Window, cx: &mut Context<Root>) {
    let s = root.state.read(cx);
    if s.search.is_none() {
        return;
    }
    let cmds = commands::matches(s);
    let ix = s.cursor.min(cmds.len().saturating_sub(1));
    if let Some(c) = cmds.into_iter().nth(ix) {
        pick(c.act, root, window, cx);
    }
}

/// Closes the bar, then acts.
pub fn pick(act: Act, root: &mut Root, window: &mut Window, cx: &mut Context<Root>) {
    root.state.update(cx, |s, cx| {
        s.search = None;
        s.cursor = 0;
        cx.notify();
    });
    match act {
        Act::Dialog => window.dispatch_action(Box::new(Open), cx),
        Act::Open(path) => root.open_paths(vec![path], cx),
        act => root.state.update(cx, |s, cx| apply(act, s, cx)),
    }
}

fn apply(act: Act, s: &mut AppState, cx: &mut Context<AppState>) {
    match act {
        Act::Go(screen) => s.navigate(screen, cx),
        Act::Select(i) => {
            s.select(i, cx);
            let screen = match s.documents.get(i) {
                Some(Document::Package { .. }) => Screen::Package,
                _ => Screen::Inspector,
            };
            s.navigate(screen, cx);
        }
        Act::Decompile => s.decompile_selected(out_dir("decompile"), cx),
        Act::Extract => s.extract_selected(out_dir("extract"), cx),
        Act::SplitQc => {
            s.split_qc = !s.split_qc;
            cx.notify();
        }
        Act::Design(d) => s.set_design(Some(d), cx),
        Act::Dialog | Act::Open(_) => {}
    }
}

/// Up and down move the highlight, wrapping at both ends.
pub fn step(root: &mut Root, delta: isize, cx: &mut Context<Root>) {
    root.state.update(cx, |s, cx| {
        let n = commands::matches(s).len() as isize;
        if s.search.is_none() || n == 0 {
            return;
        }
        s.cursor = (s.cursor as isize + delta).rem_euclid(n) as usize;
        cx.notify();
    });
}
