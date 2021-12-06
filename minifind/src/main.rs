use minifind::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Not enough arguments.\n{}", err);
        process::exit(1);
    });

    println!("Search {} in {}", config.filename, config.root_path);
    //no need to unwrap
    if let Err(e) = minifind::run(&config) {
        println!("Application error {}", e);
        process::exit(1);
    }
}
