pub use self::config::Config;

pub fn run(c: Config, input: &str) -> String {
    match c.problem() {
        "dna" => dna::run(input),
        _ => String::from("")
    }
}

mod dna;
mod config;