//! Parse of the synthetic GoldSrc v10 model (mesh, texture, frames).

#[path = "../tests/goldsrc/shared.rs"]
mod goldsrc;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let mdl = goldsrc::builder::build();
    c.bench_function("mdl_parse_goldsrc_v10", |b| {
        b.iter(|| prybar_mdl::parse(&mdl).unwrap())
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
