#[cfg(test)]
mod tests;

pub struct Config<'a> {
    problem :&'a str
}

impl<'a> Config<'a> {
    pub fn new(args: &'a[String]) -> Result<Config, &'static str>  {
        if args.len() > 1 {
            Ok(Config {problem: &args[1]})
        } else {
            Err("Need argument to specify problem!")
        }
    }

    pub fn problem(&self) -> &str {
        self.problem
    }
}
