use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    let contents = fs::read_to_string(filename).expect("Error reading file.");
    println!("Searching for: {}", query);
    println!("In file: {}", filename);
    println!("First 30 chars: {} ", &contents[..30]);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
