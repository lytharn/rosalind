#[cfg(test)]
mod tests;

use rayon::prelude::*;

pub fn run(dna: &[u8]) -> String {
    let result = dna.par_iter().map(|c| {
        match c {
            b'T' => b'U',
            _ => *c,
        }
    }).collect();
    unsafe { String::from_utf8_unchecked(result) }
}
