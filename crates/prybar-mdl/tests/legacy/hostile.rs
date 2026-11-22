use super::builder;
use prybar_mdl::parse;

#[test]
fn legacy_hostile_input_never_panics() {
    for version in [2531, 31, 37] {
        let (good, _) = builder::build(version);
        for at in (0..good.len()).step_by(4) {
            for v in [-1i32, i32::MAX, i32::MIN, 0x7fff] {
                let mut buf = good.clone();
                builder::put(&mut buf, at, v);
                let _ = parse(&buf);
            }
        }
        for len in 0..good.len() {
            let _ = parse(&good[..len]);
        }
    }
}

#[test]
fn legacy_huge_counts_fail_cleanly() {
    let (mut buf, ints) = builder::build(37);
    builder::put(&mut buf, ints + 4, i32::MAX);
    assert!(parse(&buf).is_err());
}

/// Writes the legacy fuzz seeds when asked; a no-op in normal runs.
#[test]
fn legacy_writes_fuzz_seed() {
    if let Ok(dir) = std::env::var("PRYBAR_WRITE_LEGACY_SEED") {
        for v in [2531, 31, 37] {
            std::fs::write(format!("{dir}/legacy_{v}.mdl"), builder::build(v).0).unwrap();
        }
    }
}
