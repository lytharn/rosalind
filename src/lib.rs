extern crate rayon;

pub use self::config::Config;

pub fn run(c: Config, input: &[u8]) -> String {
    match c.problem.as_ref() {
        "dna" => dna::run(input),
        "dnap" => run_in_parallel(c,|| dnap::run(input)),
        "rna" => rna::run(input),
        "rnap" => run_in_parallel(c,|| rnap::run(input)),
        "revc" => revc::run(input),
        "revcp" => run_in_parallel(c,|| revcp::run(input)),
        "subs" => subs::run(input),
        "subsp" => run_in_parallel(c, || subsp::run(input)),
        _ => String::from("")
    }
}

fn run_in_parallel<F: FnOnce() -> String>(c: Config, f: F) -> String
    where F: std::marker::Send {
    match c.threads {
        Some(threads) => {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
            pool.install(f)
        },
        None => f(),
    }
}

mod dna;
mod dnap;
mod rna;
mod rnap;
mod revc;
mod revcp;
mod subs;
mod subsp;
mod config;