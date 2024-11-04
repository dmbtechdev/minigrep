use minigrep::Config;

use std::env;
use std::io::IsTerminal;
// use std::error::Error;
// use std::env::Args;
// use std::fs;
use std::process;
use std::io::{self, BufRead};
use colored::*;

// Separation of Concerns for Binary Projects
// The organizational problem of allocating responsibility for multiple tasks 
// to the main function is common to many binary projects. As a result, 
// the Rust community has developed guidelines for splitting the separate concerns of a binary 
// program when main starts getting large. This process has the following steps:

// Split your program into a main.rs file and a lib.rs file and move your program’s logic to lib.rs.
// As long as your command line parsing logic is small, it can remain in main.rs.
// When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
// The responsibilities that remain in the main function after this process should be limited to the following:

// Calling the command line parsing logic with the argument values
// Setting up any other configuration
// Calling a run function in lib.rs
// Handling the error if run returns an error

// This pattern is about separating concerns: 
// main.rs handles running the program and 
// lib.rs handles all the logic of the task at hand. 
// Because you can’t test the main function directly, 
// this structure lets you test all of your program’s logic by moving it into functions in lib.rs. 
// The code that remains in main.rs will be small enough to verify its correctness by reading it. 

fn main() {

    // println!("{:#?}", stdin);
    // for line in stdin.lock().lines() {
    //     let line = line.expect("Could not read line from standard in");
    //     println!("{}", line);
    // }
    
    let stdin_piped = !io::stdin().is_terminal();
    let query = env::args().nth(1).unwrap_or_else(|| {
        // println!("Problem parsing arguments: {err}");

        // the error onscreen and output.txt contains nothing,
        // with cmd line: cargo run  > output.txt
        eprintln!("Problem parsing arguments: No arguments provided");
        process::exit(1); // code 1 is error, code 0 is success
    });
    // println!("{}", query);
    
    if stdin_piped {
        
        for line in io::stdin().lock().lines() {
            match line {
                Ok(line) => {
                    if line.contains(&query) {
                        println!("{}", line.replace(&query, &query.red().bold().to_string()));
                        // println!("{}", line.green());
                    }
                },
                Err(err) => {
                        eprintln!("{}{err}","Error reading line: ".red());
                        continue;
                    }
                }
                // let line = line.expect("Could not read");
                // println!("{}", line);
            }
            
        process::exit(0);
    } else {
        
    }


    // all the code that prints error messages is in one function, main.

    // std library notes:
    // The first element is traditionally the path of the executable, but it can be
    // set to arbitrary text, and might not even exist. This means this property should
    // not be relied upon for security purposes.

    // The book:
    // This matches the behavior of the arguments list in C, 
    // letting programs use the name by which they were invoked in their execution.

    // It’s often convenient to have access to the program name in case you want to print it in messages or 
    // change the behavior of the program based on what command line alias was used to invoke the program.
     
    // let args: Vec<String> = env::args().collect();

    // let args = env::args(); 
    
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    
    // Returning a Result Instead of Calling panic!
    // let config = Config::build(&args).unwrap_or_else(|err| {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {err}");

        // the error onscreen and output.txt contains nothing,
        // with cmd line: cargo run  > output.txt
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1); // code 1 is error, code 0 is success
    });

    // let config = parse_config(args);
    
    
    // dbg!(args);
    // let exe_path = &args[0];
    // let query = &args[1];
    // let file_path = &args[2];
    
    // println!("Exe file path is: \"{exe_path}\"");
    // if let Ok(config) = config {
    //     println!("Searching for the line: \"{}\"",config.query);
    //     println!("In the file path: \"{}\"",config.file_path);

    //     let contents = fs::read_to_string(config.file_path)
    //     .expect("Should have been able to read the file");
    //     println!("With text:\n{contents}");
    // }
    
    // println!("Searching for the line: \"{}\"",config.query);
    // println!("In the file path: \"{}\"",config.file_path);
    
    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        // println!("Application error: {err}");
        process::exit(1);// code 1 is error, code 0 is success
    }

    // let contents = fs::read_to_string(config.file_path)
    //     .expect("Should have been able to read the file");
    //     println!("With text:\n{contents}");

}

// Grouping Configuration Values

// At the moment, we’re returning a tuple, but then 
// we immediately break that tuple into individual parts again. 
// This is a sign that perhaps we don’t have the right abstraction yet.

// We’re not currently conveying this meaning in the structure of the data 
// other than by grouping the two values into a tuple; 
// we’ll instead put the two values into one struct and 
// give each of the struct fields a meaningful name. 

// struct Config {
//     query: String,
//     file_path: String,
// }

// impl Config {

//     // Creating a Constructor for Config
//     // fn new(args: &[String]) -> Config {
//     //     if args.len() < 3 {
//     //         panic!("not enough arguments");
//     //     }
//     //     let query = args[1].clone();
//     //     let file_path = args[2].clone();

//     //     Config { query, file_path }
//     // }

//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }

// }

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config {query, file_path}
// }

// fn parse_config(mut args: Args ) -> Config {
//     // args.skip(1); // Skip the first argument, which is the executable name
//     args.next();
//     let query = args.next().unwrap();
//     let file_path = args.next().unwrap();

//     Config {query, file_path}
// }

// fn run(config: Config) {
//     let contents = fs::read_to_string(config.file_path)
//         .expect("Should have been able to read the file");

//     println!("With text:\n{contents}");
// }

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;

//     println!("With text:\n{contents}");

//     Ok(())
// }