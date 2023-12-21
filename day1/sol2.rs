use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{BTreeMap, HashMap};

fn main() {
    let numbers = HashMap::from([
                                ("one", 1), ("two", 2),  ("three", 3), ("four", 4),
                                ("five", 5), ("six", 6), ("seven", 7), ("eight", 8),
                                ("nine", 9)
    ]);
    let contents = File::open("input.txt")
        .expect("Should have been able to read the file");
    let buf = BufReader::new(contents);
    let mut total = 0;

    for mut line in buf.lines() {
        let mut calibration: HashMap<usize, usize> = HashMap::new();
        let line_val = line.as_mut().unwrap();

        for (num, int) in &numbers {
            let pos: Vec<_> = line_val.match_indices(num).collect();
            for thing in pos {
                match thing {
                    (pos, str_int) => {
                        calibration.insert(
                            pos,
                            *numbers.get(str_int).unwrap(),
                            );
                    },
                }
            }
        }

        for (pos, chr) in line.unwrap().chars().enumerate() {
            match chr.to_digit(10) {
                Some(int) => {
                    calibration.insert(
                        pos,
                        int as usize,
                        );
                },
                None => {}
            }
        }

        let mut count_b: BTreeMap<_, _> = calibration.iter().map(|(k,v)| (k,v)).collect();
        {
            total += *count_b.first_entry().unwrap().get() * 10;
        }

        {
            total += *count_b.last_entry().unwrap().get();
        }
    }

    println!("Here's that number you asked for: {}", total);
}
