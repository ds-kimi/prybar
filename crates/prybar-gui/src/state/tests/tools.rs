use gpui::{AppContext as _, Entity, TestAppContext};

use super::fake;
use crate::state::tools::{CompileOutcome, LineKind};
use crate::state::{AppState, Screen};

pub(super) fn state(cx: &mut TestAppContext) -> Entity<AppState> {
    let mut s = AppState::default();
    s.tools.backend = fake::backend();
    cx.new(|_| s)
}

#[gpui::test]
fn compile_streams_lines_and_reports_outputs(cx: &mut TestAppContext) {
    let state = state(cx);
    state.update(cx, |s, cx| s.navigate(Screen::Compile, cx));
    cx.run_until_parked();
    state.update(cx, |s, cx| s.compile("/m/a.qc".into(), cx));
    cx.run_until_parked();
    state.read_with(cx, |s, _| {
        let run = &s.tools.compile;
        assert_eq!(run.lines.len(), 3);
        assert_eq!(run.lines[1].kind, LineKind::Warning);
        assert!(!run.running);
        let want = CompileOutcome::Built {
            outputs: vec!["/g/models/x.mdl".into()],
            warnings: 0,
        };
        assert_eq!(run.outcome, Some(want));
    });
}

#[gpui::test]
fn compile_without_games_says_why(cx: &mut TestAppContext) {
    let state = state(cx);
    state.update(cx, |s, cx| s.compile("/m/a.qc".into(), cx));
    state.read_with(cx, |s, _| {
        assert!(matches!(
            s.tools.compile.outcome,
            Some(CompileOutcome::Blocked(_))
        ));
    });
}

#[gpui::test]
fn games_detect_and_save(cx: &mut TestAppContext) {
    let state = state(cx);
    state.update(cx, |s, cx| s.navigate(Screen::Settings, cx));
    cx.run_until_parked();
    state.update(cx, |s, cx| {
        assert_eq!(s.tools.games.len(), 2);
        s.tools.game = 1;
        s.detect_games(cx);
    });
    cx.run_until_parked();
    state.update(cx, |s, cx| {
        assert_eq!(s.tools.games.len(), 3);
        let g = s.tools.games[0].clone();
        s.save_game(g, cx);
    });
    cx.run_until_parked();
    state.read_with(cx, |s, _| {
        assert_eq!((s.tools.games.len(), s.tools.game), (1, 0))
    });
}
