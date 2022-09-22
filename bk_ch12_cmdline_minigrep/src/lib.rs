use std::fs;
use std::error::Error;
use std::env;

/*
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
        //.expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
    Ok(())
}
 */

// 12-19 add search in run
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results //search(&config.query, &contents)
    { println!("{}", line); }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// lst 12-15 fail test
// lst 12-16 add search func
/*
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
 */

// lst 12-17 iterating throught each line in contents
// lst 12-18 line.contains(str)
/*
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            //
        }
    }
    vec![]
}
 */

 // lst 12-19
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::<&str>::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}

// lst 12-21
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
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
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
    // lst 12-20 new failing test for case-insensitive function
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
