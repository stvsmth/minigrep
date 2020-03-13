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
    let results = search(&config.query, &contents, config.case_sensitive);

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, is_case_sensitive: bool) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            if is_case_sensitive {
                line.contains(&query)
            } else {
                line.to_lowercase().contains(&query.to_lowercase())
            }
        })
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
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
            search(query, contents, true)
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
        assert_eq!(empty_vec, search(query, contents, true));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
    }
}
