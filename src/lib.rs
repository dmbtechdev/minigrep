use std::error::Error;
use std::fs;
use std::env;
use std::fmt::Debug;

use colored::Colorize;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {

    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    pub fn build(mut args: impl Iterator<Item=String> + Debug) -> Result<Config, &'static str> {

        // println!("{:#?}", args);
        // with iterator version
        args.next();
        let mut file_path = "".to_owned();
        let query = match args.next() {
            Some(arg1) => {
                if arg1 != "help" {
                    file_path = match args.next() {
                        Some(arg2) => arg2,
                        None => return Err("Didn't get a file path"),
                    };
                }
                arg1
            },
            None => return Err("Didn't get a query string"),
        };
        

        
        // without iterator version
        
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        // let query = args[1].clone();
        // let file_path = args[2].clone();
        
        
        // IGNORE_CASE=0 cargo run -- to poem.txt
        
        // It will return the Err variant if the environment variable is not set.
        // the ignore_case will be true or false
        // it doesn’t care about the value of the environment variable here.
        // if the user sets the env , that means, the search is case insensitive
        // if the user does not set the environment, the search is case sensitive which is default behavior in the program
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        // This will give implementation of the environment variable; 
        // if the environment variable is set to "1" or "true" then; the case insensitive search will be performed.
        // o/w, case sensitive search will be performed.
        // since the environment variable is named as "IGNORE_CASE"

        let ignore_case = env::var("IGNORE_CASE").is_ok_and(|x|x=="1" || x=="true");

        Ok(Config { query, file_path, ignore_case })
    }

}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case { 
        search_case_insensitive(&config.query, &contents)
    } else { 
        search(&config.query, &contents)
    };
    println!("Result/s: ");
    for line in results {
        println!("\t{}",line);
    }

    // println!("With text:\n{contents}");

    Ok(contents)
}

// the test-driven development (TDD) process with the following steps:

// 1. Write a test that fails and run it to make sure it fails for the reason you expect.
    // 1.1 Iterate through each line of the contents.
    // 1.2 Check whether the line contains our query string.
    // 1.3 If it does, add it to the list of values we’re returning.
    // 1.4 If it doesn’t, do nothing.
    // 1.5 Return the list of results that match.
// 2. Write or modify just enough code to make the new test pass.
// 3. Refactor the code you just added or changed and make sure the tests continue to pass.
// 4. Repeat from step 1!

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    // without iterator version
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // // vec![]
    // results

    // with iterator version
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

// Choosing Between Loops or Iterators
// Most Rust programmers prefer to use the iterator style. It’s a bit tougher to get the hang of at first, 
// but once you get a feel for the various iterator adaptors and what they do, iterators can be easier to understand.


pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        //  note that the backslash after the opening double quote tells Rust 
        // not to put a newline character at the beginning of the contents of this string literal
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
            vec!["Rust:", "Trust me."],search_case_insensitive(query, contents));
    }

}
