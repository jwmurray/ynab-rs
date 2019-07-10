// use std::env;
use std::fs;

pub fn get_token(filename: &str) -> String {
    // --snip--

    println!("In file {}", filename);

    let contents = (fs::read_to_string(filename).expect("Something went wrong reading the file"))
        .trim()
        .to_string();

    // println!("With text:\n{}", contents.trim());
    contents
}
