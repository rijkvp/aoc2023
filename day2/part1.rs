use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let limits = HashMap::<&str, u32>::from([("red", 12), ("green", 13), ("blue", 15)]);
    let mut sum = 0;
    for line in BufReader::new(stdin()).lines() {
        let line = line.unwrap();
        let (name, games) = line.split_once(":").unwrap();
        let mut possible = true;
        for subset in games.split(";") {
            let mut cubes = HashMap::new();
            for draw in subset.split(",") {
                let (count, name) = draw.trim().split_once(" ").unwrap();
                let count = count.parse::<u32>().unwrap();
                cubes.entry(name).and_modify(|c| *c += count).or_insert(count);
            }
            for (key, count) in &cubes {
                if count > &limits[key] {
                    possible = false;
                    break;
                }
            }
            if !possible {
                break;
            }
        }
        if possible {
            let game_id = name.split_once(" ").unwrap().1.parse::<u32>().unwrap();
            sum += game_id;
        }
    }
    println!("{}", sum)
}
