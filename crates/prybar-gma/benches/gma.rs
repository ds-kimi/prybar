//! Open + read every entry, and pack, on a 256-file synthetic addon.

use criterion::{criterion_group, criterion_main, Criterion};
use prybar_gma::{pack, Gma};
use std::path::Path;

fn tree(root: &Path) {
    let json = r#"{"title":"Bench","type":"tool","tags":["fun"]}"#;
    std::fs::create_dir_all(root).unwrap();
    std::fs::write(root.join("addon.json"), json).unwrap();
    for i in 0..256 {
        let p = root.join(format!("lua/d{}/f{i}.lua", i % 16));
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, vec![b'a' + (i % 26) as u8; 4096]).unwrap();
    }
}

fn bench(c: &mut Criterion) {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    tree(&src);
    let out = tmp.path().join("a.gma");
    pack(&src, &out).unwrap();
    c.bench_function("gma_open_read_all", |b| {
        b.iter(|| {
            let gma = Gma::open(&out).unwrap();
            gma.entries
                .iter()
                .map(|e| gma.read(e).unwrap().len())
                .sum::<usize>()
        })
    });
    let packed = tmp.path().join("b.gma");
    c.bench_function("gma_pack", |b| b.iter(|| pack(&src, &packed).unwrap()));
}

criterion_group!(benches, bench);
criterion_main!(benches);
