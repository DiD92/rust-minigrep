use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lower_query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&lower_query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: std::env::Args) -> Result<Config, &'static str> {
        // We don't care about the binary path
        let mut args = args.skip(1);

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string missing!"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("File to search missing!"),
        };

        let case_sensitive: bool;

        if let Some(option) = args.next() {
            case_sensitive = match CaseFormat::from_string(&option) {
                CaseFormat::Keep => true,
                CaseFormat::Ignore => false,
            };
        } else {
            case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

enum CaseFormat {
    Keep,
    Ignore,
}

impl CaseFormat {
    fn from_string(string: &str) -> CaseFormat {
        if string.starts_with("I") {
            CaseFormat::Ignore
        } else {
            CaseFormat::Keep
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
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
