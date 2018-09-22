#[cfg(test)]
mod tests;

use rayon::prelude::*;

pub fn run(dna: &[u8]) -> String {
    let (a, c, g, t) = dna.par_iter().fold(|| (0, 0, 0, 0), |mut acc, byte| {
        match byte {
            b'A' => acc.0 += 1,
            b'C' => acc.1 += 1,
            b'G' => acc.2 += 1,
            b'T' => acc.3 += 1,
            _ => (),
        };
        acc
    }).reduce(|| (0, 0, 0, 0), |mut a, b| {
        a.0 += b.0;
        a.1 += b.1;
        a.2 += b.2;
        a.3 += b.3;
        a
    });
    format!("{} {} {} {}", a, c, g, t)
}
