use std::{
    fmt::{Display, Formatter},
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone)]
struct MapRange {
    start: u64,
    end: u64,
    shift: i64,
}

impl Display for MapRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{} ({}{})",
            self.start,
            self.end,
            if self.shift < 0 { '-' } else { '+' },
            self.shift.abs(),
        )?;
        Ok(())
    }
}

impl FromStr for MapRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Ok(Self {
            start: numbers[1],
            end: numbers[1] + numbers[2],
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
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1]))
        .collect::<Vec<(u64, u64)>>();
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
    let mapping = almanac[0].clone();
    let mut next_mapping = Vec::new();
    println!("0: {:?}", almanac[0]);
    println!("1: {:?}", almanac[1]);
    // the existing one
    for first_range in &mapping {
        let mut ranges_left = almanac[1].clone();
        // for map in &almanac {
        while !ranges_left.is_empty() {
            let second_range = ranges_left.pop().unwrap();
            // the new one
            let c_start = first_range.start.clamp(second_range.start, second_range.end);
            let c_end = first_range.end.clamp(second_range.start, second_range.end);
            println!("Compare {first_range} and {second_range}");
            if c_end - c_start > 0 {
                println!("  overlap: {c_start} - {c_end}");
                // if c == a: just add the shifts together
                if c_start == second_range.start && c_end == second_range.end {
                    println!("  add together (no split)");
                    next_mapping.push(MapRange {
                        start: second_range.start,
                        end: second_range.end,
                        shift: second_range.shift + first_range.shift,
                    });
                } else {
                    // 1. The part that gets shifted
                    // c
                    println!(
                        "  add shifted: {} - {} ({})",
                        c_start,
                        c_end,
                        second_range.shift + first_range.shift
                    );
                    next_mapping.push(MapRange {
                        start: c_start,
                        end: c_end,
                        shift: second_range.shift + first_range.shift,
                    });
                    // left of c
                    if c_start > second_range.start {
                        println!(
                            "  add left split: {} - {} ({})",
                            second_range.start, c_start, second_range.shift
                        );
                        ranges_left.push(MapRange {
                            start: second_range.start,
                            end: c_start,
                            shift: second_range.shift,
                        });
                    }
                    // right of c
                    if second_range.end > c_end {
                        println!(
                            "  add right split: {} - {} ({})",
                            c_end, second_range.end, second_range.shift
                        );
                        ranges_left.push(MapRange {
                            start: c_end,
                            end: second_range.end,
                            shift: second_range.shift,
                        });
                    }
                }
            } else {
                println!("  no overlap");
            }
        }
    }
    println!("{next_mapping:?}");
    // }
    // for seed in seeds {
    //     println!("Starting with {seed:?}");
    //     let mut seed_ranges = vec![(seed)];
    //     let mut next_ranges: Vec<(u64, u64)> = Vec::new();
    //     min_location = min_location.min(location);
    // }
}
