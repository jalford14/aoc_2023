use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let contents = File::open("input.txt")
        .expect("Should have been able to read the file");
    let buf = BufReader::new(contents);
    let mut total = 0;

    for line in buf.lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;

        for chr in line.unwrap().chars() {
            match chr.to_digit(10) {
                Some(i) => {
                    if first_digit == 0 {
                        first_digit = i;
                    } else {
                        last_digit = i;
                    }
                },
                None => {}
            }
        }
        if last_digit == 0 {
            last_digit = first_digit;
        }
        total += (first_digit * 10) + last_digit;
    }

    println!("Here's that number you asked for: {}", total);
}
