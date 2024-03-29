use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {

    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = config.run() {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
