use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &mut env::Args) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("Not query string"),
        };
        let file_name = match args.next() {
            Some(q) => q,
            None => return Err("Not file_name"),
        };
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
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

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "fast";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Duct uppercase";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Duct uppercase";

        assert_eq!(
            vec!["safe, fast, productive.", "Duct uppercase"],
            search_insensitive(query, contents)
        )
    }
}
