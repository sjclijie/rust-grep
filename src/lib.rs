use std::error::Error;
use std::fs;

pub struct Config {
    filename: String,
    query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }

    fn search<'a>(&self, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(&self.query) {
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

