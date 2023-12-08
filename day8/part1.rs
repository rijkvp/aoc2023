use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut reader = BufReader::new(stdin()).lines();
    let navigation: Vec<char> = reader.next().unwrap().unwrap().chars().collect();
    reader.next();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for line in reader {
        let line = line.unwrap();
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        nodes.insert(key.to_string(), (left.to_string(), right.to_string()));
    }
    let mut location = "AAA";
    let mut steps = 0u64;
    'outer: loop {
        for direction in &navigation {
            steps += 1;
            if *direction == 'L' {
                location = &nodes[location].0;
            } else if *direction == 'R' {
                location = &nodes[location].1;
            }
            if location == "ZZZ" {
                break 'outer;
            }
        }
    }
    println!("{steps}");
}
