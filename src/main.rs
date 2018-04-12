extern crate rosalind;

use std::io;
use std::process;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|e| {
        eprintln!("Problem reading input: {}", e);
        process::exit(1);
    });
    let output = rosalind::dna::run(&input);
    println!("{}", output);
}
