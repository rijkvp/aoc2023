use std::io::{self, BufRead, BufReader};

fn main() {
    let mut sum = 0u64;
    for line in BufReader::new(io::stdin()).lines() {
        let line = line.unwrap();
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        let n = (first.to_string() + &last.to_string())
            .parse::<u64>()
            .unwrap();
        sum += n;
    }
    println!("{}", sum);
}
