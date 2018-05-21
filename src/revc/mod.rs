#[cfg(test)]
mod tests;

use std::str;

pub fn run(dna: &str) -> String {
    dna.chars().rev().map(|c| {
        match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => c
        }
    }).collect()
}