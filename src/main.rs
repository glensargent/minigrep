use std::env;
use std::process;

mod lib;
use lib::Config;

fn main() {
    // make the program take 2 inputs, a string to search for and a file to look in
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(config) {
        eprintln!("application error: {}", e);
        process::exit(1);
    }

}