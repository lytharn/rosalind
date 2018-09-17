#[cfg(test)]
mod tests;

use rayon::prelude::*;

pub fn run(input: &str) -> String {
    let mut lines = input.lines();
    let dna = lines.next().unwrap_or("");
    let substring = match lines.next() {
        Some(val) => val,
        None => return String::from("")
    };

    (0..dna.len() - substring.len() + 1).into_par_iter()
        .filter(|i| &dna[*i..i+substring.len()] == substring)
        .map(|i| (i+1).to_string())
        .reduce(|| String::new(), |mut a, b| {
            if !b.is_empty() && !a.is_empty() {
                a.push(' ');
            }
            a.push_str(&b);
            a
        })
}
