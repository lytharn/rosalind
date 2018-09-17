#[cfg(test)]
mod tests;

use rayon::prelude::*;

pub fn run(dna: &str) -> String {
    let result = dna.as_bytes().par_iter().map(|c| {
        match c {
            b'T' => b'U',
            _ => *c,
        }
    }).collect();
    String::from_utf8(result).unwrap()
}
