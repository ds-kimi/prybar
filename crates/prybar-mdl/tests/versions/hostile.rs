use super::builder;
use prybar_mdl::{embedded, parse};

#[test]
fn v53_hostile_input_never_panics() {
    let good = builder::build();
    for at in (0..good.len()).step_by(4) {
        for v in [-1i32, i32::MAX, i32::MIN, 0x7fff] {
            let mut buf = good.clone();
            buf[at..at + 4].copy_from_slice(&v.to_le_bytes());
            let _ = parse(&buf);
            let _ = embedded(&buf);
        }
    }
    for len in 0..good.len() {
        let _ = parse(&good[..len]);
        let _ = embedded(&good[..len]);
    }
}

/// Writes the v53 fuzz seed when asked; a no-op in normal runs.
#[test]
fn v53_writes_fuzz_seed() {
    if let Ok(path) = std::env::var("PRYBAR_WRITE_V53_SEED") {
        std::fs::write(path, builder::build()).unwrap();
    }
}
