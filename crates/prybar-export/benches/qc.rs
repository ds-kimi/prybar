//! QC text generation for the synthetic IR of the QC tests, with the
//! skeleton widened to a ragdoll-sized bone count.

#[path = "../tests/qc/fixture.rs"]
mod fixture;
#[path = "../tests/qc/fixture_anim.rs"]
mod fixture_anim;
#[path = "../tests/qc/fixture_flex.rs"]
mod fixture_flex;
#[path = "../tests/qc/parts.rs"]
mod parts;

use criterion::{criterion_group, criterion_main, Criterion};
use prybar_export::qc::{qc_files, Names, QcOptions};

fn bench(c: &mut Criterion) {
    let mut ir = fixture::ir();
    let extra = (0..128).map(|i| prybar_mdl::ir::Bone {
        name: format!("bone{i}"),
        parent: 0,
        ..ir.bones[0].clone()
    });
    ir.bones.extend(extra.collect::<Vec<_>>());
    let names = Names::new(&ir);
    c.bench_function("qc_write", |b| {
        b.iter(|| qc_files(&ir, &names, QcOptions::default()))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
