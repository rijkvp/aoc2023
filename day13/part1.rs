use std::io::{stdin, BufRead, BufReader};

fn is_reflection(pattern: &Vec<Vec<char>>, row: usize) -> bool {
    let mut reflection = true;
    for i in 0..pattern.len() {
        let idx_a = row as isize - i as isize;
        let idx_b = row + 1 + i;
        if idx_a < 0 || idx_b >= pattern.len() {
            break;
        }
        let idx_a = idx_a as usize;
        let a = &pattern[idx_a];
        let b = &pattern[idx_b];
        if a != b {
            println!("Rows {} and {} are NOT identical", idx_a, idx_b);
            reflection = false;
            break;
        }
    }
    reflection
}

fn find_reflection(pattern: &Vec<Vec<char>>) -> Option<usize> {
    let mut prev = &pattern[0];
    for (n, row) in pattern[1..].iter().enumerate() {
        if prev == row {
            println!("Found identical rows {} and {}", n, n + 1);
            if is_reflection(pattern, n) {
                println!("Found reflection! {}", n + 1);
                return Some(n + 1);
            }
        }
        prev = row;
    }
    None
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn count(pattern: &Vec<Vec<char>>) -> usize {
    if let Some(rows) = find_reflection(&pattern) {
        rows * 100
    } else if let Some(cols) = find_reflection(&transpose(pattern.clone())) {
        cols
    } else {
        unreachable!()
    }
}

fn main() {
    let mut pattern = Vec::new();
    let mut sum = 0;
    for line in BufReader::new(stdin()).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            sum += count(&pattern);
            pattern.clear();
        } else {
            pattern.push(line.chars().collect());
        }
    }
    sum += count(&pattern);
    println!("{sum}");
}
