use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut pairs = Vec::new();
    let mut empty_rows = Vec::new();
    let mut set_cols = Vec::new();
    for (r, line) in BufReader::new(stdin()).lines().enumerate() {
        let line = line.unwrap();
        let mut empty = true;
        if r == 0 {
            set_cols = vec![false; line.len()];
        }
        for (c, char) in line.chars().enumerate() {
            if char == '#' {
                empty = false;
                pairs.push((r as isize, c as isize));
                set_cols[c] = true;
            }
        }
        if empty {
            empty_rows.push(r as isize);
        }
    }
    for pair in pairs.iter_mut() {
        let mut shift = 0;
        for r in empty_rows.iter() {
            if pair.0 > *r {
                shift += 1;
            }
        }
        pair.0 += shift;
    }
    for pair in pairs.iter_mut() {
        let mut shift = 0;
        for c in 0..set_cols.len() {
            if !set_cols[c] {
                if pair.1 > c as isize {
                    shift += 1;
                }
            }
        }
        pair.1 += shift;
    }
    let mut sum = 0;
    for i in 0..pairs.len() {
        for j in i + 1..pairs.len() {
            let (r1, c1) = pairs[i];
            let (r2, c2) = pairs[j];
            let sp = (r2 - r1).abs() + (c2 - c1).abs();
            sum += sp;
        }
    }
    println!("{}", sum);
}
