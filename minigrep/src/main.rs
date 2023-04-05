use std::{env, process};

use minigrep::{self, Config};

fn main() {
    let config = Config::new(&mut env::args()).unwrap_or_else(|err| {
        eprintln!("Parsing config error: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
