use std::error::Error;
use std::fs;

pub struct Config {
    filename: String,
    query: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't  get a filename string"),
        };

        Ok(Config { query, filename })
    }

    fn search<'a>(&self, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        let query = self.query.to_lowercase();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line)
            }
        }

        results
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(&self.filename)?;

        for ret in self.search(&contents) {
            println!("{}", ret);
        }

        Ok(())
    }
}

