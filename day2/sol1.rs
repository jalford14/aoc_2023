use std::io;
use regex::Regex;

fn main() {
    regex = Regex::new(r"(Game \d*: )(.*)");
    println!("{:?}", regex);
}
