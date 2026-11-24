use super::{frames, message};
use crate::{Event, MAX_MESSAGE};

#[test]
fn message_drops_paths_files_quotes_and_user() {
    let raw = r#"failed to read /Users/kimi/games/crate.mdl: "props/crate" by kimi at C:\Users\kimi\x, see body.vvd."#;
    let out = message(raw, Some("kimi"));
    assert!(!out.contains("kimi"), "{out}");
    assert!(!out.contains("crate"), "{out}");
    assert!(!out.contains("body"), "{out}");
    assert!(
        out.starts_with("failed to read <path> <redacted> by <user> at <path>"),
        "{out}"
    );
    assert_eq!(
        message("index 7 out of range 1.5", None),
        "index 7 out of range 1.5"
    );
    let people =
        "steam 76561198012345678 item 3141592653 mail kimi@x.fr from 192.168.1.5 at 10.0.0.1:";
    assert_eq!(
        message(people, None),
        "steam <number> item <number> mail <email> from <number> at <number>"
    );
}

#[test]
fn message_is_capped_on_a_char_boundary() {
    let out = message(&"é".repeat(400), None);
    assert!(out.len() <= MAX_MESSAGE);
}

#[test]
fn frames_keep_prybar_symbols_only() {
    let trace = "   0: std::panicking::begin\n   1: prybar_mdl::vvd::read::h0123abcd\n             at /Users/kimi/src/vvd.rs:12:5\n   2: gpui::app::run\n   3: prybar_gui::main";
    assert_eq!(frames(trace), ["prybar_mdl::vvd::read", "prybar_gui::main"]);
}

#[test]
fn events_round_trip_and_reject_extra_fields() {
    let ev = Event::crash("0.1.0", "boom".into(), vec!["prybar_gui::main".into()]);
    let json = serde_json::to_string(&ev).expect("serializes");
    assert_eq!(serde_json::from_str::<Event>(&json).ok(), Some(ev.clone()));
    assert!(ev.valid());
    let extra = r#"{"event":"app_start","version":"1","os":"macos","arch":"x","user":"k"}"#;
    assert!(serde_json::from_str::<Event>(extra).is_err());
    let bad = Event::crash("0.1.0", "x".into(), vec!["std::rt".into()]);
    assert!(!bad.valid());
}
