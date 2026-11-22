//! Wiring done once on a real launch; tests build the state by hand.

use gpui::Context;

use crate::state::AppState;

/// Real launch only: the store, the kill switches, the crash hook (only
/// when analytics are on), then the launch event.
pub fn launch(state: &mut AppState, cx: &mut Context<AppState>) {
    let p = &mut state.privacy;
    p.store = crate::telemetry::default_dir().map(prybar_telemetry::Store::new);
    p.endpoint = prybar_telemetry::ENDPOINT;
    p.blocked = prybar_telemetry::blocked();
    if let (true, Some(store)) = (p.on, p.store.clone()) {
        prybar_telemetry::report::install(store, crate::update::VERSION);
    }
    state.start_telemetry(cx);
}

impl AppState {
    pub(super) fn reload_sent_log(&mut self) {
        if let Some(store) = &self.privacy.store {
            self.privacy.log = store.entries();
        }
    }
}
