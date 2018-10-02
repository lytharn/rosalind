#[cfg(test)]
mod tests;

use rayon::prelude::*;

pub fn run(input: &[u8]) -> String {
    let mut lines = input.split(|b|b == &b'\n');
    let dna = lines.next().unwrap_or(&[]);
    let substring = match lines.next() {
        Some(&[]) => return String::from(""),
        Some(val) => val,
        None => return String::from("")
    };

    if dna.len() < substring.len() {
        return String::from("")
    }

    (0..dna.len() - substring.len() + 1).into_par_iter()
        .filter(|i| &dna[*i..i+substring.len()] == substring)
        .map(|i| (i+1).to_string())
        .reduce(|| String::new(), |a, b| {
            if !b.is_empty() && !a.is_empty() {
                return a + " " + &b
            }
            a + &b
        })
}
