use std::{env, error::Error, fs};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("Incorrect program usage! Please supply two arguments: a query and a path");
        }

        let query = args[1].as_str();
        let file_path = args[2].as_str();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &text)
    } else {
        search(&config.query, &text)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    return seach_closure(
        query,
        text,
        |query, _, line, _, result| {
            if line.contains(query) {
                result.push(line);
            }
        },
    );
}

pub fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();

    return seach_closure(
        query,
        text,
        |query, _, line, _, result| {
            if line.to_lowercase().contains(query) {
                result.push(line);
            }
        },
    );
}

fn seach_closure<'a, R>(
    query: &str,
    text: &'a str,
    line_adder: R,
) -> Vec<&'a str>
where
    R: FnOnce(&str, &Vec<&'a str>, &'a str, usize, &mut Vec<&'a str>) -> () + Copy,
{
    let mut result = Vec::new();
    let lines: Vec<&str> = text.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        line_adder(&query, &lines, line, i, &mut result);
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
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
