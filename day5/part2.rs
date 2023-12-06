use std::{
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone)]
struct MapRange {
    start: i64,
    end: i64,
    shift: i64,
}

impl FromStr for MapRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        Ok(Self {
            start: numbers[1],
            end: numbers[1] + numbers[2] - 1,
            shift: numbers[0] as i64 - numbers[1] as i64,
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
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1] - 1))
        .collect::<Vec<(i64, i64)>>();
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
        let range = MapRange::from_str(&line).unwrap();
        current_map.push(range);
    }
    if !current_map.is_empty() {
        almanac.push(current_map);
    }
    let mut seeds = seeds;
    let mut next_seeds: Vec<(i64, i64)> = Vec::new();
    for map in &almanac {
        next_seeds.clear();
        while let Some((start, end)) = seeds.pop() {
            let mut mapped = false;
            for range in map {
                // Simple case: the location lies entirely inside the range
                if start >= range.start && end <= range.end {
                    next_seeds.push((
                        // The entire location is shifted without splitting
                        start + range.shift,
                        end + range.shift,
                    ));
                    mapped = true;
                    break;
                } else {
                    // The location is not entirely inside the range
                    let clamped_start = start.clamp(range.start, range.end);
                    let clamped_end = end.clamp(range.start, range.end);
                    if clamped_end > clamped_start {
                        // The location overlaps with the range
                        next_seeds.push((clamped_start + range.shift, clamped_end + range.shift));
                        // The remaining parts might still be shifted by another range
                        // Left split
                        if start < clamped_start {
                            seeds.push((start, clamped_start));
                        }
                        // Right split
                        if end > clamped_end {
                            seeds.push((clamped_end, end));
                        }
                        mapped = true;
                        break;
                    }
                }
            }
            if !mapped {
                next_seeds.push((start, end));
            }
        }
        seeds = next_seeds.clone();
    }
    let min_location = seeds
        .into_iter()
        .min_by_key(|(start, _)| *start)
        .map(|(start, _)| start)
        .unwrap();
    println!("{}", min_location);
}
