//! The analytics flow against a temp store and a fake transport: nothing
//! leaves before the notice is answered, off deletes what waits, and the
//! log shows every payload. No network.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use gpui::{Entity, Modifiers, TestAppContext, VisualTestContext};
use prybar_telemetry::{Event, Store};

use crate::state::{AppState, Screen};

static SENT: AtomicUsize = AtomicUsize::new(0);
/// Tests run in parallel and share `SENT`; one send from another test
/// between a read and an assert made the counts flaky.
static SERIAL: Mutex<()> = Mutex::new(());

/// A first launch: notice not answered, one crash report left from before.
fn first_launch(
    cx: &mut TestAppContext,
) -> (tempfile::TempDir, Entity<AppState>, VisualTestContext) {
    let (state, mut cx) = crate::designs::prybar::tests::window(cx, false);
    let dir = tempfile::tempdir().expect("tempdir");
    let store = Store::new(dir.path().join("telemetry"));
    store
        .queue(&Event::crash("0.1.0", "boom".into(), vec![]))
        .expect("queued");
    state.update(&mut cx, |s, cx| {
        s.prefs = Some(dir.path().join("gui.txt"));
        s.privacy.notice_seen = false;
        s.privacy.store = Some(store);
        s.privacy.endpoint = "https://collector.invalid/v1/event";
        s.privacy.transport = |_, _| {
            SENT.fetch_add(1, Ordering::SeqCst);
            Ok(())
        };
        s.start_telemetry(cx);
    });
    cx.run_until_parked();
    (dir, state, cx)
}

/// "Keep on" sends the waiting crash and the launch event, logs both as
/// sent, hides the notice and remembers the answer.
#[gpui::test]
fn keep_on_sends_only_after_the_answer(cx: &mut TestAppContext) {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let before = SENT.load(Ordering::SeqCst);
    let (dir, state, mut cx) = first_launch(cx);
    assert!(cx.debug_bounds("pb-privacy-notice").is_some());
    let none = SENT.load(Ordering::SeqCst) == before;
    assert!(none, "nothing before the answer");
    let at = cx.debug_bounds("pb-privacy-keep").expect("keep button");
    cx.simulate_click(at.center(), Modifiers::none());
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-privacy-notice").is_none());
    let sent = state.read_with(&cx, |s, _| {
        s.privacy.log.iter().filter(|e| e.status == "sent").count()
    });
    assert_eq!(sent, 2);
    let prefs = crate::state::prefs::load(&dir.path().join("gui.txt"));
    assert!(prefs.telemetry && prefs.notice_seen);
}

/// "Turn off" deletes the waiting crash report and never sends.
#[gpui::test]
fn turn_off_deletes_the_queue(cx: &mut TestAppContext) {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let (dir, state, mut cx) = first_launch(cx);
    let before = SENT.load(Ordering::SeqCst);
    let at = cx.debug_bounds("pb-privacy-off").expect("off button");
    cx.simulate_click(at.center(), Modifiers::none());
    cx.run_until_parked();
    let pending = state.read_with(&cx, |s, _| {
        s.privacy.store.as_ref().map(|st| st.pending().len())
    });
    assert_eq!(pending, Some(0));
    assert_eq!(SENT.load(Ordering::SeqCst), before);
    let prefs = crate::state::prefs::load(&dir.path().join("gui.txt"));
    assert!(!prefs.telemetry && prefs.notice_seen);
    assert!(!state.read_with(&cx, |s, _| s.privacy.sending()));
}

/// Settings › Privacy: a row opens its JSON, the switch turns off,
/// "Clear log" empties the table; a kill switch says why it is off.
#[gpui::test]
fn settings_show_log_and_switch(cx: &mut TestAppContext) {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let (_dir, state, mut cx) = first_launch(cx);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Settings, cx));
    cx.run_until_parked();
    let at = cx
        .debug_bounds("pb-sent-0")
        .expect("queued crash is listed");
    cx.simulate_click(at.center(), Modifiers::none());
    cx.run_until_parked();
    assert_eq!(state.read_with(&cx, |s, _| s.privacy.open), Some(0));
    let at = cx.debug_bounds("pb-telemetry").expect("switch");
    cx.simulate_click(at.center(), Modifiers::none());
    cx.run_until_parked();
    assert!(!state.read_with(&cx, |s, _| s.privacy.on));
    let at = cx.debug_bounds("pb-telemetry-clear").expect("clear");
    cx.simulate_click(at.center(), Modifiers::none());
    cx.run_until_parked();
    assert!(state.read_with(&cx, |s, _| s.privacy.log.is_empty()));
    state.update(&mut cx, |s, _| {
        s.privacy.blocked = Some("DO_NOT_TRACK is set")
    });
    let status = state.read_with(&cx, |s, _| (s.privacy.sending(), s.privacy.status()));
    assert_eq!(status, (false, "Off: DO_NOT_TRACK is set".to_string()));
}
