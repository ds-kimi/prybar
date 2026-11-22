use super::builder;
use prybar_mdl::{inspect, parse};

#[test]
fn goldsrc_hostile_input_never_panics() {
    let good = builder::build();
    for at in (0..good.len()).step_by(2) {
        for v in [-1i32, i32::MAX, i32::MIN, 0x7fff] {
            let mut buf = good.clone();
            builder::put(&mut buf, at.min(good.len() - 4), v);
            let _ = parse(&buf);
            let _ = inspect(&buf);
        }
    }
    for len in 0..good.len() {
        let _ = parse(&good[..len]);
    }
}

#[test]
fn goldsrc_v04_v06_report_unsupported() {
    for v in [4, 6] {
        let mut buf = builder::build();
        builder::put(&mut buf, 4, v);
        assert!(parse(&buf).is_err());
    }
}

#[test]
fn goldsrc_writes_fuzz_seed() {
    if let Ok(dir) = std::env::var("PRYBAR_WRITE_GOLDSRC_SEED") {
        std::fs::write(format!("{dir}/goldsrc_v10.mdl"), builder::build()).unwrap();
    }
}
