#[cfg(test)]
mod tests;

use std::path::PathBuf;

pub struct Config {
    pub file_path: PathBuf,
    pub problem: String,
    pub threads: Option<usize>,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = PathBuf::from(match args.next() {
            Some(arg) => arg,
            None => return Err("Need first argument to specify file path!")
        });

        let problem = match args.next() {
            Some(arg) => arg,
            None => return Err("Need second argument to specify problem!"),
        };

        let threads: Option<usize> = match args.next().map(|s| {
            match s.parse() {
                Ok(0) | Err(_) => return Err("Invalid number of threads!"),
                Ok(threads) => Ok(threads),
            }
        }) {
            Some(Err(e)) => return Err(e),
            Some(Ok(threads)) => Some(threads),
            _ => None,
        };

        Ok(Config {file_path, problem, threads})
    }
}
