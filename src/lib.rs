extern crate rayon;

pub use self::config::Config;

pub fn run(c: Config, input: &str) -> String {
    match c.problem() {
        "dna" => dna::run(input),
        "dnap" => dnap::run(input),
        "rna" => rna::run(input),
        "rnap" => rnap::run(input),
        "revc" => revc::run(input),
        "subs" => subs::run(input),
        _ => String::from("")
    }
}

mod dna;
mod dnap;
mod rna;
mod rnap;
mod revc;
mod subs;
mod config;