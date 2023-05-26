use std::cell::{RefCell, RefMut};
use std::collections::HashSet;
use std::fmt::Display;
use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
    pub line_numbers: bool,
    pub context: (usize, usize),
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Self, Box<dyn Error>> {
        if args.len() < 3 {
            return Err(MyErr::boxed("Incorrect program usage! Please supply two arguments: a query and a path and optional flags preceded by -"));
        }

        let mut query = None;
        let mut file_path = None;
        let mut ignore_case = false;
        let mut line_numbers = false;
        let mut context = (0, 0);

        let args: Vec<RefCell<Arg>> = args
            .into_iter()
            .skip(1)
            .map(|text| RefCell::new(Arg::new(text)))
            .collect();

        for i in 0..args.len() {
            let mut arg = args[i].borrow_mut();
            if arg.consumed {
                continue;
            }

            let flag = Flag::parse(&arg, args.get(i + 1))?;

            if let Some(flag) = flag {
                match flag {
                    Flag::CaseInsensitve => ignore_case = true,
                    Flag::LineNumber => line_numbers = true,
                    Flag::Before(n) => context.0 = n,
                    Flag::After(n) => context.1 = n,
                    Flag::Context(n) => {
                        context.0 = n;
                        context.1 = n;
                    }
                }
            } else {
                if query.is_none() {
                    query = Some(arg.text);
                } else if file_path.is_none() {
                    file_path = Some(arg.text);
                } else {
                    return Err(MyErr::boxed(
                        "Too many arguments! Please suply the query and a path.",
                    ));
                }
            }

            arg.consumed = true;
        }

        let query = query.ok_or(MyErr::boxed("Please supply two non flag arguments!"))?;

        let file_path = file_path.ok_or(MyErr::boxed("Please supply two non flag arguments!"))?;

        let ignore_case_env = env::var("IGNORE_CASE").is_ok();
        if ignore_case_env {
            ignore_case = true;
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
            line_numbers,
            context,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file_path)?;

    for line in search(&text, &config) {
        println!("{line}");
    }

    Ok(())
}

pub fn search(text: &str, config: &Config) -> Vec<String> {
    let query = if config.ignore_case {
        config.query.to_lowercase()
    } else {
        String::from(config.query)
    };

    let line_printer = if config.line_numbers {
        |line, i| format!("{}: {}", i + 1, line)
    } else {
        |line, _| String::from(line)
    };

    let should_add = if config.ignore_case {
        |query: &str, line: &str, _| line.to_lowercase().contains(query)
    } else {
        |query: &str, line: &str, _| line.contains(query)
    };

    seach_closure(&query, text, should_add, line_printer, config.context)
}

pub fn seach_closure<'a, R, S>(
    query: &str,
    text: &'a str,
    mut should_add: R,
    mut line_printer: S,
    context: (usize, usize),
) -> Vec<String>
where
    R: FnMut(&str, &'a str, usize) -> bool,
    S: FnMut(&'a str, usize) -> String,
{
    let mut result = Vec::new();
    let mut lines_to_add = HashSet::new();
    let lines: Vec<&str> = text.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        if should_add(query, line, i) {
            for j in i.saturating_sub(context.0)..=std::cmp::min(i + context.1, lines.len() - 1) {
                lines_to_add.insert(j);
            }
        }
    }

    let mut lines_to_add: Vec<usize> = lines_to_add.into_iter().collect();
    lines_to_add.sort_unstable();

    for i in lines_to_add {
        result.push(line_printer(lines[i], i));
    }

    result
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Flag {
    CaseInsensitve,
    LineNumber,
    Before(usize),
    After(usize),
    Context(usize),
}

impl Flag {
    fn parse(
        arg: &RefMut<Arg>,
        next: Option<&RefCell<Arg>>,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        if arg.text.len() != 2 || !arg.text.starts_with("-") {
            return Ok(None);
        }

        match arg.text {
            "-i" => Ok(Some(Flag::CaseInsensitve)),
            "-a" => {
                let number = read_usize_flag("-a", next)?;
                Ok(Some(Flag::After(number)))
            }
            "-b" => {
                let number = read_usize_flag("-b", next)?;
                Ok(Some(Flag::Before(number)))
            }
            "-c" => {
                let number = read_usize_flag("-c", next)?;
                Ok(Some(Flag::Context(number)))
            }
            "-n" => Ok(Some(Flag::LineNumber)),
            _ => Err(MyErr::boxed(&format!("Flag {} doesnt exist", arg.text))),
        }
    }
}

#[derive(Debug)]
struct Arg<'a> {
    text: &'a str,
    consumed: bool,
}

impl<'a> Arg<'a> {
    fn new(text: &'a str) -> Self {
        Arg {
            text,
            consumed: false,
        }
    }
}

#[derive(Debug)]
struct MyErr {
    message: String,
}

impl MyErr {
    fn new(message: &str) -> Self {
        MyErr {
            message: message.to_owned(),
        }
    }

    fn boxed(message: &str) -> Box<Self> {
        Box::new(Self::new(message))
    }
}

impl Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.message))
    }
}

impl Error for MyErr {}

fn read_usize_flag(flag_name: &str, arg: Option<&RefCell<Arg>>) -> Result<usize, Box<dyn Error>> {
    match arg {
        None => Err(MyErr::boxed(&format!(
            "Please provide a number argument for the {} flag",
            flag_name
        ))),
        Some(arg) => {
            let mut arg: RefMut<Arg> = arg.borrow_mut();
            let number: usize = arg.text.parse()?;
            arg.consumed = true;

            Ok(number)
        },
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn mock_config<'a>(query: &'a str) -> Config<'a> {
        let empty: &'static str = "";

        Config {
            query,
            file_path: empty,
            ignore_case: false,
            line_numbers: false,
            context: (0, 0),
        }
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let config = mock_config(query);

        assert_eq!(vec!["safe, fast, productive."], search(contents, &config));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let mut config = mock_config(query);
        config.ignore_case = true;

        assert_eq!(vec!["Rust:", "Trust me."], search(contents, &config));
    }

    #[test]
    fn context() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let mut config = mock_config(query);

        config.context = (1, 1);
        assert_eq!(vec!["Rust:", "safe, fast, productive.", "Pick three."], search(contents, &config));

        config.context = (10, 0);
        assert_eq!(vec!["Rust:", "safe, fast, productive."], search(contents, &config));

        config.context = (0, 10);
        assert_eq!(vec!["safe, fast, productive.", "Pick three.", "Trust me."], search(contents, &config));
    }

    #[test]
    fn line_number() {
        let query = "I am";
        let contents = "\
I am who I am
Too bad I'm not
who I need to be";

        let mut config = mock_config(query);
        config.line_numbers = true;
        assert_eq!(vec!["1: I am who I am"], search(contents, &config));
    }
}
