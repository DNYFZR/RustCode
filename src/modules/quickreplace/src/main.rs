// File systems & command line tools
// Project : Build a search & replace tool

use std::env;
use std::fs;
use text_colorizer::*;
use regex::Regex;

// Arg base class
#[derive(Debug)] 
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String
}

// User info
fn print_usage() {
    eprintln!("{} - change occurences of one string into another", "quickreplace".green());
    eprintln!("Usage : quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

// Process CLI args
fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() !=4 {
        print_usage();

        eprintln!(
            "{} wrong number of arguments: expected 4 but got {}", 
            "Error".red().bold(), 
            args.len());

        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

// Regex replace
fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

// App
fn main() {
    // parse input
    let args = parse_args();
    
    // read file
    let data = match fs::read_to_string(&args.filename) {Ok(v) => v, Err(e) => {
        eprintln!("{} failed to read from file '{}' : {:?}", "Error:".red().bold(), args.filename, e);
        std::process::exit(1);
    }};

    // Modify file content
    let replaced_data = match replace(&args.target, &args.replacement, &data) {Ok(v) => v, Err(e) => {
        eprintln!("{} failed to replace text : {:?}", "Error:".red().bold(), e);
        std::process::exit(1);
    }};

    // write new file 
    match fs::write(&args.output, &replaced_data) {Ok(v) => {}, Err(e) => {
        eprintln!("{} failed to write to file '{}' : {:?}", "Error:".red().bold(), args.filename, e);
        std::process::exit(1);
    }};
}
