use std::io::{BufRead, BufReader};
use std::io::stdin;

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

fn find(line: &str, i: usize) -> Option<u32> {
    let c = line.chars().nth(i).unwrap();
    let mut d = None;
    if c.is_numeric() {
      d = c.to_digit(10)
    } else {
    for (p, n) in DIGITS {
      if line[i..line.len()].starts_with(p) {
        d = Some(*n);
        break;
      }
    } 
  }
  d
}

fn find_digit(line: &str, rev: bool) -> Option<u32> {
  if rev {
    for i in (0..line.len()).rev() {
      if let Some(d) = find(line, i) {
        return Some(d);
      };
    }
  } else {
    for i in 0..line.len() {
      if let Some(d) = find(line, i) {
        return Some(d);
      };
    }
  };
  None
}

fn main() {
    let file = stdin().lock();
    let mut sum = 0u32;
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        println!("line: {}", line);
        let first = find_digit(&line, false).unwrap();
        let last = find_digit(&line, true).unwrap();
        println!("{} & {}", first, last);
        let n = format!("{}{}", first, last).parse::<u32>().unwrap();
        sum += n;
    }
    println!("Sum: {}", sum);
}
