use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // skip over app name
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query param"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename param"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

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
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
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
Duct Tape.
";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn n_results() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive
better than duct tape
Pick three.
";

        assert_eq!(
            vec!["safe, fast, productive", "better than duct tape"],
            search(query, contents)
        );
    }

    #[test]
    fn no_result() {
        let query = "virus";
        let empty_vec: Vec<&str> = Vec::new();
        let contents = "\
Rust: 
safe, fast, productive
Pick three.
";
        assert_eq!(empty_vec, search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
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
