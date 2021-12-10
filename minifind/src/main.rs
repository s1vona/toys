use minifind::Config;
use std::{env, process};
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Arguments parsing error.\n{}", err);
        process::exit(1);
    });

    println!("Search {} in {}", config.filename, config.root_path);
    //no need to unwrap
    if let Err(e) = minifind::run(&config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
