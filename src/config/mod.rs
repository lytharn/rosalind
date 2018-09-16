#[cfg(test)]
mod tests;

pub struct Config {
    pub problem: String,
    pub threads: Option<usize>,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let problem = match args.next() {
            Some(arg) => arg,
            None => return Err("Need argument to specify problem!"),
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

        Ok(Config {problem, threads})
    }
}
