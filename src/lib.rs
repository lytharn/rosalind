mod dna;
pub mod config;

pub fn run(c: config::Config, input: &str) -> String {
    match c.problem() {
        "dna" => dna::run(input),
        _ => String::from("")
    }
}
