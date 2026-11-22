//! Kill switches that win over the in-app setting.

/// Why sending is forced off, if it is: `DO_NOT_TRACK` set to anything but
/// `0`, `PRYBAR_TELEMETRY=0`, or any CI environment.
pub fn blocked_by(get: impl Fn(&str) -> Option<String>) -> Option<&'static str> {
    let set = |k: &str| get(k).is_some_and(|v| !v.is_empty() && v != "0" && v != "false");
    if set("DO_NOT_TRACK") {
        return Some("DO_NOT_TRACK is set");
    }
    if get("PRYBAR_TELEMETRY").is_some_and(|v| v == "0" || v == "false" || v == "off") {
        return Some("PRYBAR_TELEMETRY=0 is set");
    }
    // A CI runner is never a person using the app.
    set("CI").then_some("running in CI")
}

/// `blocked_by` on the real process environment.
pub fn blocked() -> Option<&'static str> {
    blocked_by(|k| std::env::var(k).ok())
}

#[cfg(test)]
mod tests {
    use super::blocked_by;

    #[test]
    fn each_switch_blocks_and_zero_values_do_not() {
        let with =
            |k: &'static str, v: &'static str| blocked_by(move |q| (q == k).then(|| v.to_string()));
        assert!(with("DO_NOT_TRACK", "1").is_some());
        assert!(with("PRYBAR_TELEMETRY", "0").is_some());
        assert!(with("CI", "true").is_some());
        assert!(with("DO_NOT_TRACK", "0").is_none());
        assert!(with("PRYBAR_TELEMETRY", "1").is_none());
        assert!(blocked_by(|_| None).is_none());
    }
}
