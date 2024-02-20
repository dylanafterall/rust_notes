use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // 'static is reserved lifetime name. means data pointed to by the reference lives for
    //  remaining lifetime of the running program
    //  the string literal will be stored in read-only memory of the binary
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // check that user entered two command line arguments
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // ignoring &args[0], which will be name of binary
        // using inefficient clone(). arguments are simple strings, and copies are only made once
        let query = args[1].clone();
        let file_path = args[2].clone();

        // use .is_ok() instead of .unwrap() or .expect() because we don't care about the value
        //  of the IGNORE_CASE environment variable, instead only if it is or isn't set
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> is trait object (a type that implements the Error trait)
    // ? will return the value from current function for caller to handle
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    // if successful, return unit type ()
    Ok(())
}

// cfg attribute enables conditional compiling
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // backslash after opening quote tells compiler not to put newline character at beginning of string literal
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

// data returned by search function will live as long as data passed in as 'contents' argument
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // .lines() returns iterator to line-by-line collection of string slices
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str, ) -> Vec<&'a str> {
    let query = query.to_lowercase(); let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
