#[cfg(test)]
mod tests;

use rayon::prelude::*;

pub fn run(dna: &str) -> String {
    dna.par_chars().map(|c| {
        match c {
            'T' => 'U',
            _ => c,
        }
    }).collect()
}
