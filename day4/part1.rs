use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut sum = 0;
    for line in BufReader::new(stdin()).lines() {
        let line = line.unwrap();
        let (_, nums) = line.split_once(':').unwrap();
        let (win_nums, own_nums) = nums.split_once('|').unwrap();
        let win_nums = win_nums
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<HashSet<u64>>();
        let own_nums = own_nums
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<HashSet<u64>>();
        let mut count = 0;
        for num in own_nums {
            if win_nums.contains(&num) {
                count += 1;
            }
        }
        if count > 0 {
            sum += 1 << (count - 1);
        }
    }
    println!("{sum}")
}
