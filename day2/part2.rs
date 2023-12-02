use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut sum = 0;
    for line in BufReader::new(stdin()).lines() {
        let line = line.unwrap();
        let (_, games) = line.split_once(":").unwrap();
        let mut max_cubes = HashMap::<&str, u64>::new();
        for subset in games.split(";") {
            for draw in subset.split(",") {
                let (count, name) = draw.trim().split_once(" ").unwrap();
                let count = count.parse::<u64>().unwrap();
                max_cubes
                    .entry(name)
                    .and_modify(|c| *c = count.max(*c))
                    .or_insert(count);
            }
        }
        let mut power = 1;
        for num in max_cubes.values() {
            power *= num;
        }
        sum += power;
    }
    println!("{}", sum)
}
