//! Open + read every entry, and pack, on a 256-file synthetic tree.

use criterion::{criterion_group, criterion_main, Criterion};
use prybar_vpk::{pack, PackOptions, Vpk};
use std::path::Path;

const OPTS: PackOptions = PackOptions {
    version: 2,
    chunk_size: None,
};

fn tree(root: &Path) {
    for i in 0..256 {
        let p = root.join(format!("d{}/f{i}.bin", i % 16));
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, vec![i as u8; 4096]).unwrap();
    }
}

fn bench(c: &mut Criterion) {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    tree(&src);
    let out = tmp.path().join("a.vpk");
    pack(&src, &out, &OPTS).unwrap();
    c.bench_function("vpk_open_read_all", |b| {
        b.iter(|| {
            let vpk = Vpk::open(&out).unwrap();
            vpk.entries
                .iter()
                .map(|e| vpk.read(e).unwrap().len())
                .sum::<usize>()
        })
    });
    let packed = tmp.path().join("b.vpk");
    c.bench_function("vpk_pack", |b| {
        b.iter(|| pack(&src, &packed, &OPTS).unwrap())
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
