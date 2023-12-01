use std::io::stdin;
use std::io::{BufRead, BufReader};

const DIGITS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let mut sum = 0u32;
    for line in BufReader::new(stdin()).lines() {
        let line = line.unwrap();
        let mut min = None;
        let mut first_digit = None::<u32>;
        let mut max = None;
        let mut last_digit = None::<u32>;
        for (pattern, digit) in DIGITS {
            if let Some(pos) = line.find(pattern) {
                if min.is_none() || Some(pos) < min {
                    min = Some(pos);
                    first_digit = Some(*digit);
                }
            }
            if let Some(pos) = line.rfind(pattern) {
                if max.is_none() || Some(pos) > max {
                    max = Some(pos);
                    last_digit = Some(*digit);
                }
            }
        }
        for (pos, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                if max.is_none() || Some(pos) > max {
                    max = Some(pos);
                    last_digit = ch.to_digit(10);
                }
                if min.is_none() || Some(pos) < min {
                    min = Some(pos);
                    first_digit = ch.to_digit(10);
                }
            }
        }
        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            let num = (first.to_string() + &last.to_string())
                .parse::<u32>()
                .unwrap();
            sum += num;
        } else {
            eprintln!("Error: no digits found on line: {}", line);
        }
    }
    println!("{}", sum);
}
