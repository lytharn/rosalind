extern crate rosalind;

use std::env;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = rosalind::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|e| {
        eprintln!("Problem reading input: {}", e);
        process::exit(1);
    });

    let output = rosalind::run(config, &input);
    println!("{}", output);
}
