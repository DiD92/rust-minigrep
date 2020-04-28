use std::{env, error::Error, fs};

use clap::{App, Arg};

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
    pub fn from_clap_app() -> Result<Config, &'static str> {
        let matches = App::new("Rust - Minigrep")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Dídac S. <didac.semente@gmail.com>")
            .about("'grep' command implementation in Rust")
            .arg(
                Arg::with_name("ignore_case")
                    .help("Ignore case distinctions")
                    .short("i")
                    .long("ignore-case"),
            )
            .arg(
                Arg::with_name("keep_case")
                    .help("Keep case distinctions, takes precedence over --ignore-case")
                    .short("k")
                    .long("keep-case"),
            )
            .arg(
                Arg::with_name("EXPRESSION")
                    .help("Expression to search for")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("FILE")
                    .help("File to be searched")
                    .required(true),
            )
            .get_matches();

        let ignore_case = matches.is_present("ignore_case");
        let keep_case = matches.is_present("keep_case");

        let expression = match matches.value_of("EXPRESSION") {
            Some(expr) => expr,
            None => return Err("Query string missing!"),
        };

        let file = match matches.value_of("FILE") {
            Some(expr) => expr,
            None => return Err("File to search missing!"),
        };

        Ok(Config {
            query: String::from(expression),
            filename: String::from(file),
            case_sensitive: keep_case || (!ignore_case && env::var("CASE_INSENSITIVE").is_err()),
        })
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
