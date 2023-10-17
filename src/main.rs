use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|_| {
        eprintln!("Usage: minigrep [query] [file]");
        process::exit(0);
    });

    if let Err(_) = minigrep::run(config) {
        eprintln!("ERROR: could not read file",);
        process::exit(1);
    }
}
