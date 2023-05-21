use std::{error::Error, fs};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("Incorrect program usage! Please supply two arguments: a query and a path");
        }

        let query = args[1].as_str();
        let file_path = args[2].as_str();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &text) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in text.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
