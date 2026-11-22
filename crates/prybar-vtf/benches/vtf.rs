//! Decoding speed of a 1024x1024 DXT5 and BGRA8888 mip 0.

use criterion::{criterion_group, criterion_main, Criterion};
use prybar_vtf::Vtf;

#[path = "../tests/support/mod.rs"]
mod support;

fn bench(c: &mut Criterion) {
    let dxt5 = support::vtf(5, 15, (1024, 1024), 1, &vec![0x5a; 1024 * 1024]);
    let vtf = Vtf::parse(dxt5).unwrap();
    c.bench_function("vtf_dxt5_1024", |b| {
        b.iter(|| vtf.decode(0).unwrap().rgba.len())
    });
    let bgra = support::vtf(2, 12, (1024, 1024), 1, &vec![0x5a; 1024 * 1024 * 4]);
    let vtf = Vtf::parse(bgra).unwrap();
    c.bench_function("vtf_bgra_1024", |b| {
        b.iter(|| vtf.decode(0).unwrap().rgba.len())
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
