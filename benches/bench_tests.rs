#![feature(test)]
extern crate test;
extern crate rosalind;

use test::{Bencher, black_box};

fn run(problem: &str, input: &[u8]) -> String {
    let args = vec!["rosalind".to_string(), "file_path".to_string(), problem.to_string()];
    let config = rosalind::Config::new(args.into_iter()).unwrap();
    rosalind::run(config, input)
}

fn run_with_threads(problem: &str, threads: &str, input: &[u8]) -> String {
    let args = vec!["rosalind".to_string(), "file_path".to_string(), problem.to_string(), threads.to_string()];
    let config = rosalind::Config::new(args.into_iter()).unwrap();
    rosalind::run(config, input)
}

fn generate_dna(n: usize) -> impl Iterator<Item=u8> {
    (0..n).map(|n| {
        match n%4 {
            0 => b'A',
            1 => b'T',
            2 => b'G',
            _ => b'C'
        }
    })
}

#[bench]
fn bench_dna(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run("dna", &dna));
    });
}

#[bench]
fn bench_dnap1(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run_with_threads("dnap", "1", &dna));
    });
}

#[bench]
fn bench_dnap2(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run_with_threads("dnap", "2", &dna));
    });
}

#[bench]
fn bench_dnap4(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run_with_threads("dnap", "4", &dna));
    });
}

#[bench]
fn bench_dnapx(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run("dnap", &dna));
    });
}

#[bench]
fn bench_rna(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run("rna", &dna));
    });
}

#[bench]
fn bench_rnap1(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run_with_threads("rnap", "1", &dna));
    });
}

#[bench]
fn bench_rnap2(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run_with_threads("rnap", "2", &dna));
    });
}

#[bench]
fn bench_rnap4(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run_with_threads("rnap", "4", &dna));
    });
}

#[bench]
fn bench_rnapx(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run("rnap", &dna));
    });
}

#[bench]
fn bench_revc(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run("revc", &dna));
    });
}

#[bench]
fn bench_revcp1(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run_with_threads("revcp", "1", &dna));
    });
}

#[bench]
fn bench_revcp2(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run_with_threads("revcp", "2", &dna));
    });
}

#[bench]
fn bench_revcp4(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run_with_threads("revcp", "4", &dna));
    });
}

#[bench]
fn bench_revcpx(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run("revcp", &dna));
    });
}

#[bench]
fn bench_subs(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000)
        .chain(generate_dna(40))
        .collect();

    b.iter(|| {
        black_box(run("subs", &dna));
    });
}

#[bench]
fn bench_subsp1(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000)
        .chain(generate_dna(40))
        .collect();

    b.iter(|| {
        black_box(run_with_threads("subsp", "1", &dna));
    });
}


#[bench]
fn bench_subsp2(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000)
        .chain(generate_dna(40))
        .collect();

    b.iter(|| {
        black_box(run_with_threads("subsp", "2", &dna));
    });
}

#[bench]
fn bench_subsp4(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000)
        .chain(generate_dna(40))
        .collect();

    b.iter(|| {
        black_box(run_with_threads("subsp", "4", &dna));
    });
}


#[bench]
fn bench_subspx(b: &mut Bencher) {
    let dna: Vec<_> = generate_dna(10*1000*1000)
        .chain(generate_dna(40))
        .collect();

    b.iter(|| {
        black_box(run("subsp", &dna));
    });
}