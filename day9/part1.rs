use std::io::BufRead;

pub fn differentiate(input: Vec<i64>) -> Vec<Vec<i64>> {
    let mut result = vec![input];
    loop {
        let current = result.last().unwrap();
        let mut next = Vec::new();
        let mut constant = true;
        for i in 1..current.len() {
            let diff = current[i] - current[i - 1];
            next.push(diff);
            if diff != 0 {
                constant = false;
            }
        }
        result.push(next);
        if constant {
            break;
        }
    }
    result
}

fn extrapolate(mut sequences: Vec<Vec<i64>>) -> i64 {
    let index = sequences.len() - 1;
    let mut last = 0;
    sequences[index].push(0);
    for i in (0..index).rev() {
        let before = *sequences[i].last().unwrap();
        let value = before + last;
        sequences[i].push(value);
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
        let next_value = extrapolate(differentiate(sequence));
        sum += next_value;
    }
    println!("{sum}");
}
