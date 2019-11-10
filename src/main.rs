use std::env;
use std::process;

use mg::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error occured: {}, abort", err);
        process::exit(1);
    });

    if let Err(e) = mg::run(config) {
        eprintln!("Error occured: {}, abort", e);
        process::exit(1);
    };
}
