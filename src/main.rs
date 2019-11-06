use std::env;
use std::process;

use mg::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error occured: {}, abort", err);
        process::exit(1);
    });

    if let Err(e) = mg::run(config) {
        println!("Error occured: {}, abort", e);
        process::exit(1);
    };
}
