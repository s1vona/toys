use glob::glob;
use std::error::Error;
use std::fs;
pub struct Config {
    pub root_path: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: minifind <path> <filename>");
        }
        let root_path = args[1].clone();
        let filename = args[2].clone();
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
