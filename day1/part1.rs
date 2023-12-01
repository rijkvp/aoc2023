use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let mut sum = 0u64;
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        let n = format!("{}{}", first, last).parse::<u64>().unwrap();
        sum += n;
    }
    println!("Sum: {}", sum);
}
