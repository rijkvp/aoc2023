mod part1;
use std::io::BufRead;

fn extrapolate_previous(mut sequences: Vec<Vec<i64>>) -> i64 {
    let index = sequences.len() - 1;
    let mut last = 0;
    sequences[index].push(0);
    for i in (0..index).rev() {
        let before = *sequences[i].first().unwrap();
        let value = before - last;
        sequences[i].insert(0, value);
        last = value;
    }
    last
}

fn main() {
    let mut sum = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let sequence: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let next_value = extrapolate_previous(part1::differentiate(sequence));
        sum += next_value;
    }
    println!("{sum}");
}
