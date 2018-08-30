#![feature(test)]
extern crate test;
extern crate rosalind;

use test::{Bencher, black_box};

fn run(arg: &str, input: &str) -> String {
    let args = vec![String::from("rosalind"), String::from(arg)];
    let config = rosalind::Config::new(&args).unwrap();
    rosalind::run(config, input)
}

#[bench]
fn bench_dna(b: &mut Bencher) {
    let dna: String = (0..10000000).map(|n| {
        match n%4 {
            0 => 'A',
            1 => 'T',
            2 => 'G',
            _ => 'C'
        }
    }).collect();

    b.iter(|| {
        black_box(run("dna", &dna));
    });
}
