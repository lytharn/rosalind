#[cfg(test)]
mod tests;

pub fn run(input: &[u8]) -> String {
    let mut lines = input.split(|b|b == &b'\n');
    let dna = lines.next().unwrap_or(&[]);
    let substring = match lines.next() {
        Some(&[]) => return String::from(""),
        Some(val) => val,
        None => return String::from("")
    };

    (0..dna.len()-substring.len() + 1)
        .filter(|i| &dna[*i..i+substring.len()] == substring)
        .map(|i| (i+1).to_string())
        .fold(String::from(""), |acc, s| {
            match acc.as_ref() {
                "" => s,
                _ => acc + " " + &s
            }
        })
}
