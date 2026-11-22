//! Parse + geometry load of the synthetic v49 model, and anim decode. Real-model batch
//! benches come with the round-trip harness (prybar-roundtrip).

#[path = "../tests/anim/fixture/mod.rs"]
mod anim;
#[path = "../tests/common/mod.rs"]
mod common;
#[path = "../tests/versions/builder.rs"]
mod v53;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let (mdl, vvd, vtx) = (
        common::build(),
        common::geometry::vvd(),
        common::geometry::vtx(),
    );
    c.bench_function("mdl_inspect", |b| {
        b.iter(|| prybar_mdl::inspect(&mdl).unwrap())
    });
    c.bench_function("mdl_parse_geometry", |b| {
        b.iter(|| {
            let mut ir = prybar_mdl::parse(&mdl).unwrap();
            let vvd = prybar_mdl::parse_vvd(&vvd).unwrap();
            let vtx = prybar_mdl::parse_vtx(&vtx).unwrap();
            prybar_mdl::load_geometry(&mut ir, &vvd, &vtx).unwrap();
            ir
        })
    });
}

/// Animation decode alone: the synthetic model with its stream in the MDL.
fn bench_anim(c: &mut Criterion) {
    let mdl = anim::build(false);
    c.bench_function("mdl_parse_anim", |b| {
        b.iter(|| prybar_mdl::parse(&mdl).unwrap())
    });
}

fn bench_v53(c: &mut Criterion) {
    let mdl = v53::build();
    c.bench_function("mdl_parse_v53", |b| {
        b.iter(|| prybar_mdl::parse(&mdl).unwrap())
    });
}

criterion_group!(benches, bench, bench_anim, bench_v53);
criterion_main!(benches);
