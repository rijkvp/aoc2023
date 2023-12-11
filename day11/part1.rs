use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut pairs = Vec::new();
    let mut empty_rows = Vec::new();
    let mut set_cols = Vec::new();
    for (r, line) in BufReader::new(stdin()).lines().enumerate() {
        let line = line.unwrap();
        println!("{}", line);
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
    for r in empty_rows.iter() {
        for pair in pairs.iter_mut() {
            if pair.0 > *r {
                pair.0 += 1;
            }
        }
    }
    for c in 0..set_cols.len() {
        if !set_cols[c] {
            println!("Expanding column {}", c);
            for pair in pairs.iter_mut() {
                if pair.1 > c as isize {
                    println!("Shifting pair {:?} to the right", pair);
                    pair.1 += 1;
                }
            }
        }
    }
    println!("Expanded:");
    let mut count = 0;
    for r in 0..12 {
        for c in 0..13 {
            if pairs.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    for i in 0..pairs.len() {
        for j in i + 1..pairs.len() {
            let (r1, c1) = pairs[i];
            let (r2, c2) = pairs[j];
            let sp = (r2 - r1).abs() + (c2 - c1).abs();
            println!("{i}-{j} = {sp}");
            count += sp;
        }
    }
    println!("count: {}", count);
}
