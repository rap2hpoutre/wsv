extern crate rustc_serialize;
extern crate docopt;
extern crate regex;

use regex::Regex;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::process;
use docopt::Docopt;

const USAGE: &'static str = "
Whathever separated values. 
Replace comma (or something else) by semicolon (or something else) in a so-called CSV.

Usage:
  wsv [-s <s>] [-d <d>] <source> <dest>
  wsv (-h | --help)

Options:   
  -s <s>        Source separator [default: ,]
  -d <d>        Destination separator [default: ;]
  -h --help     Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_s: bool,
    flag_d: bool,
    arg_s: Option<String>,
    arg_d: Option<String>,
    arg_source: String,
    arg_dest: String,
}

// Whathever Separated Values main function
fn main() {
    // Get args
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    // Source text
    let mut s = file_get_contents(&args.arg_source);

    // Destination text
    let after = build_regex( &args.arg_s.unwrap_or_else(|| ",".to_string()) )
        .replace_all(&mut s, &*match args.arg_d {
            Some(x) => x + &"$1".to_string(),
            None    => ";$1".to_string(),
        });

    // Write file
    file_put_contents(&args.arg_dest, &after);
}

// Build regex
fn build_regex(separator: &str) -> Regex {
    let r = separator.to_string() + r#"([^""# + &separator.to_string() + r#",\r\n]+|"(?:[^"]|"")*")?"#;
    Regex::new(&r).unwrap()
}

// Get file contents as string
fn file_get_contents(filename: &str) -> String {
    let mut s = String::new();
    match open_or_die(filename).read_to_string(&mut s) {
        Err(why) => die(&format!("Couldn't read file {}: {}", filename, why.description())),
        Ok(_) => s,
    }
}

// Put a string into a file
fn file_put_contents(filename: &str, contents: &str) {
    match create_or_die(filename).write_all(contents.as_bytes()) {
        Err(why) => die(&format!("Couldn't write file {}: {}", filename, why.description())),
        Ok(_) => println!("Successfully wrote to {}", filename),
    }
}


// Open a file or die
fn open_or_die(filename: &str) -> File {
    File::open(&Path::new(filename))
        .unwrap_or_else(|why| die(&format!("Couldn't open file {}: {}", filename, why.description())))
}

// Create a file or die
fn create_or_die(filename: &str) -> File {
    File::create(&Path::new(filename))
        .unwrap_or_else(|why| die(&format!("Couldn't create file {}: {}", filename, why.description())))
}

// Die with message
fn die(message: &str) -> ! {
    println!("{}", message);
    process::exit(1);
}