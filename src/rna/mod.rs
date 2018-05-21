#[cfg(test)]
mod tests;

pub fn run(dna: &str) -> String {
    dna.replace("T", "U")
}
