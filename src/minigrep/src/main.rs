use std::env; // for using command line arguments
use std::process;

use minigrep::Config;

fn main() {
    // env::args() returns iterator to command line arguments
    // .collect() returns collection from that iterator
    // will panic if any argument has invalid unicode
    // in cases where invalid unicode must be accepted, use std::env::args_os
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        // stop program immediately and return number passed as exit status code
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // not using .unwrap_or_else() because run() returns unit type upon success (nothing to unwrap)
    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}