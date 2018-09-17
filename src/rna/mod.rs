#[cfg(test)]
mod tests;

pub fn run(dna: &str) -> String {
    let result = dna.as_bytes().iter().map(|c| {
        match *c {
            b'T' => b'U',
            _ => *c,
        }
    }).collect();
    String::from_utf8(result).unwrap()
}
