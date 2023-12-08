use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

#[derive(Debug, Copy, Clone)]
struct Node {
    index: usize,
    end: bool,
}

fn read_input() -> (Vec<char>, Vec<Node>, Vec<(Node, Node)>) {
    let mut reader = BufReader::new(stdin()).lines();
    let navigation: Vec<char> = reader.next().unwrap().unwrap().chars().collect();
    reader.next();
    let mut names: HashMap<String, Node> = HashMap::new();
    let mut input: Vec<(String, String)> = Vec::new();
    let mut locations: Vec<Node> = Vec::new();
    for (i, line) in reader.enumerate() {
        let line = line.unwrap();
        input.push((line[7..10].to_string(), line[12..15].to_string()));
        let key = line[0..3].to_string();
        let node = Node {
            index: i,
            end: key.ends_with('Z'),
        };
        if key.ends_with('A') {
            locations.push(node);
        }
        names.insert(key, node);
    }
    let mut nodes: Vec<(Node, Node)> = Vec::new();
    for (left, right) in input {
        nodes.push((names[&left], names[&right]));
    }
    (navigation, locations, nodes)
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn main() {
    let (navigation, locations, nodes) = read_input();
    let mut steps = Vec::new();
    for start in locations {
        let mut location = start;
        let mut current_steps = 0u64;
        while !location.end {
            for dir in &navigation {
                current_steps += 1;
                if *dir == 'L' {
                    location = nodes[location.index].1;
                } else if *dir == 'R' {
                    location = nodes[location.index].1;
                }
            }
        }
        steps.push(current_steps);
    }
    if let Some(res) = steps.into_iter().reduce(|acc, x| lcm(acc, x)) {
        println!("{res}")
    }
}
