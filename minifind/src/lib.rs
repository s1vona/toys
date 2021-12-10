use glob::glob;
use std::error::Error;
use std::fs;
pub struct Config {
    pub root_path: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: minifind <path> <filename>");
        }
        args.next();
        let root_path = match args.next() {
            Some(s) => s,
            None => return Err("Didn't have a path."),
        };
        let filename = match args.next() {
            Some(s) => s,
            None => return Err("Didn't have a filename."),
        };
        Ok(Config {
            root_path,
            filename,
        })
    }
}
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let full_path = format!("{}/{}", config.root_path, config.filename);
    for entry in glob(&full_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", fs::canonicalize(path)?),
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(())
}

#[cfg(tests)]
mod tests {
    use super::*;
}
