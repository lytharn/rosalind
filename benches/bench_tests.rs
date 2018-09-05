#![feature(test)]
extern crate test;
extern crate rosalind;

use test::{Bencher, black_box};

fn run(arg: &str, input: &str) -> String {
    let args = vec![String::from("rosalind"), String::from(arg)];
    let config = rosalind::Config::new(&args).unwrap();
    rosalind::run(config, input)
}

fn generate_dna(n: usize) -> impl Iterator<Item=char> {
    (0..n).map(|n| {
        match n%4 {
            0 => 'A',
            1 => 'T',
            2 => 'G',
            _ => 'C'
        }
    })
}

#[bench]
fn bench_dna(b: &mut Bencher) {
    let dna: String = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run("dna", &dna));
    });
}

#[bench]
fn bench_dnap(b: &mut Bencher) {
    let dna: String = generate_dna(10*1000*1000).collect();

    b.iter(|| {
        black_box(run("dnap", &dna));
    });
}

#[bench]
fn bench_rna(b: &mut Bencher) {
    let dna: String = generate_dna(1000*1000).collect();

    b.iter(|| {
        black_box(run("rna", &dna));
    });
}

#[bench]
fn bench_rnap(b: &mut Bencher) {
    let dna: String = generate_dna(1000*1000).collect();

    b.iter(|| {
        black_box(run("rnap", &dna));
    });
}

#[bench]
fn bench_revc(b: &mut Bencher) {
    let dna: String = generate_dna(1000*1000).collect();

    b.iter(|| {
        black_box(run("revc", &dna));
    });
}

#[bench]
fn bench_revcp(b: &mut Bencher) {
    let dna: String = generate_dna(1000*1000).collect();

    b.iter(|| {
        black_box(run("revcp", &dna));
    });
}

#[bench]
fn bench_subs(b: &mut Bencher) {
    let dna: String = generate_dna(100*1000)
        .chain(("\n".to_string()+&"ATGC".repeat(10)).chars())
        .collect();

    b.iter(|| {
        black_box(run("subs", &dna));
    });
}

#[bench]
fn bench_subsp(b: &mut Bencher) {
    let dna: String = generate_dna(100*1000)
        .chain(("\n".to_string()+&"ATGC".repeat(10)).chars())
        .collect();

    b.iter(|| {
        black_box(run("subsp", &dna));
    });
}