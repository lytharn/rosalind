#[cfg(test)]
mod tests;

pub fn run(dna: &[u8]) -> String {
    let result = dna.iter().rev().map(|c| {
        match *c {
            b'A' => b'T',
            b'T' => b'A',
            b'C' => b'G',
            b'G' => b'C',
            _ => *c
        }
    }).collect();
    unsafe { String::from_utf8_unchecked(result) }
}