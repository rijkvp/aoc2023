use std::io::{stdin, BufRead, BufReader};

fn spring_groups(spings: &Vec<char>) -> Vec<usize> {
    let mut current = 0;
    let mut groups = Vec::new();
    for s in spings {
        if *s == '#' {
            current += 1;
        } else if *s != '#' && current > 0 {
            groups.push(current);
            current = 0;
        }
    }
    if current > 0 {
        groups.push(current);
    }
    groups
}

// Recursive brute force to try all posibilities
pub fn count_arrangements(springs: &mut Vec<char>, groups: &Vec<usize>) -> u64 {
    if let Some(p) = springs.iter().position(|c| *c == '?') {
        // Recursive case, replace next '?'
        springs[p] = '#';
        let c1 = count_arrangements(springs, groups);
        springs[p] = '.';
        let c2 = count_arrangements(springs, groups);
        springs[p] = '?'; // backtracking
        return c1 + c2;
    }
    // Base case, all ? filled
    if spring_groups(&springs) == *groups {
        1
    } else {
        0
    }
}

pub fn read_input() -> Vec<(Vec<char>, Vec<usize>)> {
    let lines = BufReader::new(stdin())
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    lines
        .iter()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(l, r)| {
            (
                l.chars().collect::<Vec<char>>(),
                r.split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect()
}

fn main() {
    let input = read_input();
    let count: u64 = input
        .into_iter()
        .map(|(mut s, g)| count_arrangements(&mut s, &g))
        .sum();
    println!("{count}")
}
