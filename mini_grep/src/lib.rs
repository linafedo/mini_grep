use std::{env, fs};
use std::error::Error;
use std::process;
use env::var;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(vec: Vec<String>) -> Result<Config, &'static str> {
        if vec.len() < 3 {
            return Result::Err("You must pass 2 parameters")
        }
        let ignore_case = var("IGNORE_CASE").is_ok();
        Ok(Config { query: vec[1].clone(), file_path: vec[2].clone(), ignore_case})
    }
}

pub fn run(vec: Vec<String>) {
    let config = Config::build(vec).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(err) = get_text_from_file(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}

fn get_text_from_file(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search_case_sensitive(&config.query, &contents) {
            println!("{line}");
        }
    }


    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            vec.push(line);
        }
    }
    vec
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec: Vec<&str> = vec![];
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            vec.push(line);
        }
    }
    vec
}

#[cfg(test)]
const TEST_CONTENTS: &str= "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, TEST_CONTENTS)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, TEST_CONTENTS)
        );
    }
}
