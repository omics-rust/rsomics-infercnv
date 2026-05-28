use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_infercnv(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-infercnv");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let counts = manifest.join("tests/golden/counts.tsv");
    let genes = manifest.join("tests/golden/genes.gtf");
    let normals = manifest.join("tests/golden/normals.txt");
    c.bench_function("rsomics-infercnv golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .args([
                    "--matrix",
                    counts.to_str().unwrap(),
                    "--gtf",
                    genes.to_str().unwrap(),
                    "--normal-cells",
                    normals.to_str().unwrap(),
                ])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_infercnv);
criterion_main!(benches);
