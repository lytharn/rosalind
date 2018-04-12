#[cfg(test)]
mod tests;

pub fn run(dna: &str) -> String {
    let (mut a, mut c, mut g, mut t) = (0, 0, 0, 0);
    for byte in dna.bytes() {
        match byte {
            b'A' => a += 1,
            b'C' => c += 1,
            b'G' => g += 1,
            b'T' => t += 1,
            _ => (),
        }
    }
    format!("{} {} {} {}", a, c, g, t)
}
