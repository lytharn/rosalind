extern crate rosalind;
extern crate memmap;

use memmap::MmapOptions;
use std::env;
use std::fs::File;
use std::process;

fn main() {
    let config = rosalind::Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    let file = File::open(&config.file_path).unwrap_or_else(|e| {
        eprintln!("Problem reading file: {}", e);
        process::exit(1);
    });
    let input = unsafe { MmapOptions::new().map(&file).unwrap_or_else(|e| {
        eprintln!("Problem memory mapping file: {}", e);
        process::exit(1);
    }) };

    let output = rosalind::run(config, &input);
    println!("{}", output);
}
