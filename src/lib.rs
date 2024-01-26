use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;    
    let found = if config.case_sensitive {
        search(&config.query, &contents)
    }
    else {
        search_case_insensitive(&config.query, &contents)
    };
    
    println!("{:?}", found);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Both query and filename must be provided as command line arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{ query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines_found = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            lines_found.push(line);
        }
    }

    lines_found
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines_found = Vec::new();
    let query = query.to_uppercase();


    for line in contents.lines() {
        if line.to_uppercase().contains(&query) {
            lines_found.push(line);
        }
    }

    lines_found
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
        assert_eq!(vec!["Rust:", "Trust menx."], search_case_insensitive(query, contents));
    }
}