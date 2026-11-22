//! Parse of the synthetic legacy models (2531 and v37).

#[path = "../tests/legacy/shared.rs"]
mod legacy;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    for v in [2531, 37] {
        let (mdl, _) = legacy::builder::build(v);
        c.bench_function(&format!("mdl_parse_legacy_{v}"), |b| {
            b.iter(|| prybar_mdl::parse(&mdl).unwrap())
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
