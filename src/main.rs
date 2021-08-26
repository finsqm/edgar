use std::env;
use std::process;

use edgar::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Will format all SQL found in {}", config.path);

    if let Err(e) = edgar::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}