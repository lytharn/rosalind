pub use self::config::Config;

pub fn run(c: Config, input: &str) -> String {
    match c.problem() {
        "dna" => dna::run(input),
        "rna" => rna::run(input),
        "revc" => revc::run(input),
        _ => String::from("")
    }
}

mod dna;
mod rna;
mod revc;
mod config;