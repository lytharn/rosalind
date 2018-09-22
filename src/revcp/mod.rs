#[cfg(test)]
mod tests;

use rayon::prelude::*;

pub fn run(dna: &[u8]) -> String {
    let result = dna.par_iter().rev().map(|c| {
        match c {
            b'A' => b'T',
            b'T' => b'A',
            b'C' => b'G',
            b'G' => b'C',
            _ => *c
        }
    }).collect();
    String::from_utf8(result).unwrap()
}