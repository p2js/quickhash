use std::{env, process};
use quickhash::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("Argument error: {e}");
        process::exit(1);
    });

    if let Err(err) = quickhash::run(config) {
        eprintln!("Error: {err}");
    }
}
