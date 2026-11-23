//! The update flow against a fake release source: button states, the
//! restart path and a failed download, without network or install.

use std::sync::atomic::{AtomicBool, Ordering};

use gpui::{Modifiers, TestAppContext};
use prybar_update::{Release, Status, Verified};

use super::{Backend, Phase};
use crate::state::Screen;

static RESTARTED: AtomicBool = AtomicBool::new(false);

const RELEASE: &[u8] = br#"{"tag_name":"v99.0.0","html_url":"https://example.invalid/r",
    "body":"Faster decompile.","assets":[]}"#;

fn backend(ok: bool) -> Backend {
    Backend {
        check: || Status::Available(Release::parse(RELEASE).expect("fixture parses")),
        download: if ok {
            |r, progress| {
                progress(5, Some(10));
                let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
                let archive = dir.path().join("prybar.tar.gz");
                Ok(Verified {
                    release: r.clone(),
                    archive,
                    dir,
                })
            }
        } else {
            |_, _| Err("checksum mismatch for prybar.tar.gz".into())
        },
        restart: |_| {
            RESTARTED.store(true, Ordering::SeqCst);
            Ok(())
        },
    }
}

/// Manual mode: the green button offers the version, a click downloads,
/// then it offers the restart, which installs through the backend.
#[gpui::test]
fn button_walks_available_ready_restart(cx: &mut TestAppContext) {
    let (state, mut cx) = crate::designs::prybar::tests::window(cx, false);
    state.update(&mut cx, |s, cx| {
        s.updates.backend = backend(true);
        s.updates.auto = false;
        s.check_updates(false, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        state.read_with(&cx, |s, _| s.updates.phase.clone()),
        Phase::Available
    );
    let at = cx.debug_bounds("pb-update").expect("update button drawn");
    cx.simulate_click(at.center(), Modifiers::none());
    cx.run_until_parked();
    let (phase, screen) = state.read_with(&cx, |s, _| (s.updates.phase.clone(), s.screen));
    assert_eq!((phase, screen), (Phase::Ready, Screen::Settings));
    assert!(
        cx.debug_bounds("pb-auto-update").is_some(),
        "settings shows updates"
    );
    let at = cx.debug_bounds("pb-update").expect("restart button drawn");
    cx.simulate_click(at.center(), Modifiers::none());
    cx.run_until_parked();
    assert!(
        RESTARTED.load(Ordering::SeqCst),
        "restart reached the backend"
    );
}

/// Auto mode downloads on its own; a failure says why, offers the link
/// and hides the green button.
#[gpui::test]
fn failed_download_shows_reason_and_link(cx: &mut TestAppContext) {
    let (state, mut cx) = crate::designs::prybar::tests::window(cx, true);
    state.update(&mut cx, |s, cx| {
        s.updates.backend = backend(false);
        s.check_updates(false, cx);
        s.navigate(Screen::Settings, cx);
    });
    cx.run_until_parked();
    let phase = state.read_with(&cx, |s, _| s.updates.phase.clone());
    assert!(matches!(phase, Phase::Failed(ref e) if e.contains("checksum")));
    assert!(cx.debug_bounds("pb-update").is_none());
    assert!(cx.debug_bounds("pb-update-link").is_some());
    // Settings may toast its games list after it; order is not the point.
    let said = state.read_with(&cx, |s, _| {
        s.toasts.iter().any(|t| t.text.contains("Update failed"))
    });
    assert!(said);
}

/// Up to date: no button, and turning auto off is remembered.
#[gpui::test]
fn up_to_date_keeps_bar_quiet(cx: &mut TestAppContext) {
    let (state, mut cx) = crate::designs::prybar::tests::window(cx, false);
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("gui.txt");
    state.update(&mut cx, |s, cx| {
        s.prefs = Some(path.clone());
        s.updates.backend.check = || Status::UpToDate;
        s.check_updates(true, cx);
        s.set_auto_update(false, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        state.read_with(&cx, |s, _| s.updates.phase.clone()),
        Phase::UpToDate
    );
    assert!(cx.debug_bounds("pb-update").is_none());
    assert!(!crate::state::prefs::load(&path).auto_update);
}
