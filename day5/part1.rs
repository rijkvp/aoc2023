use std::{
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone)]
struct Range {
    dest: u64,
    source: u64,
    len: u64,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Ok(Self {
            dest: numbers[0],
            source: numbers[1],
            len: numbers[2],
        })
    }
}

fn main() {
    let mut reader = BufReader::new(stdin()).lines();
    let first_line = reader.next().unwrap().unwrap();
    let seeds = first_line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    reader.next();
    let mut almanac = Vec::new();
    let mut current_map = Vec::new();
    for line in reader {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        } else if line.ends_with(':') {
            if !current_map.is_empty() {
                almanac.push(current_map.clone());
                current_map.clear();
            }
            continue;
        }
        let range = Range::from_str(&line).unwrap();
        current_map.push(range);
    }
    if !current_map.is_empty() {
        almanac.push(current_map);
    }
    let mut min_location = u64::MAX;
    for seed in seeds {
        let mut location = seed;
        for map in &almanac {
            for range in map {
                if location >= range.source && location < range.source + range.len {
                    location = location + range.dest - range.source;
                    break;
                }
            }
        }
        min_location = min_location.min(location);
    }
    println!("{min_location}");
}
