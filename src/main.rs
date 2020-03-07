use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let mut file = File::open(filename).expect("Error opening file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file.");
    println!("Searching for: {}", query);
    println!("In file: {}", filename);
    println!("First 30 chars: {} ", &contents[..30]);
}
