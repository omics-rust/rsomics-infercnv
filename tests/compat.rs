use std::process::Command;

fn ours() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rsomics-infercnv"))
}

fn golden(n: &str) -> String {
    format!("{}/tests/golden/{}", env!("CARGO_MANIFEST_DIR"), n)
}

// No CLI upstream (R inferCNV is a library, not a CLI). Self-correctness: the CNV
// signal must reflect expression relative to the reference cells — reference cells
// sit near baseline (~0), and a high-expression cell scores above a low-expression
// one and above the reference.
#[test]
fn cnv_reflects_relative_expression() {
    let out = ours()
        .args([
            "--matrix",
            &golden("counts.tsv"),
            "--gtf",
            &golden("genes.gtf"),
            "--ref-cells",
            &golden("normals.txt"),
        ])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let s = String::from_utf8_lossy(&out.stdout);
    let mut data = s.lines().filter(|l| l.contains('\t'));
    let header: Vec<&str> = data.next().unwrap().split('\t').collect();
    let col = |name: &str| header.iter().position(|h| *h == name).unwrap();
    let (c2, c3, n1, n2) = (col("CELL2"), col("CELL3"), col("NORMAL1"), col("NORMAL2"));

    let first: Vec<&str> = data.next().unwrap().split('\t').collect();
    let v = |i: usize| first[i].parse::<f64>().unwrap();

    let n_genes = s.lines().filter(|l| l.starts_with("GENE_")).count();
    assert_eq!(n_genes, 5, "5 genes expected in CNV matrix");
    assert!(
        v(n1).abs() < 0.2 && v(n2).abs() < 0.2,
        "reference cells should sit near baseline"
    );
    assert!(
        v(c2) > v(c3),
        "high-expression cell must score above low-expression cell"
    );
    assert!(v(c2) > v(n1), "tumor cell must score above the reference");
}
