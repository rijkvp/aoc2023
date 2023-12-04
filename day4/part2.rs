use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut cards = Vec::<usize>::new();
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
        let mut count = 0usize;
        for num in own_nums {
            if win_nums.contains(&num) {
                count += 1;
            }
        }
        cards.push(count);
    }
    let mut copies = vec![0; cards.len()];
    let mut sum = 0;
    for i in 0..cards.len() {
        let start = i + 1;
        let end = (i + cards[i]).min(cards.len() - 1);
        let instances = 1 + copies[i];
        for j in start..=end {
            copies[j] += instances;
        }
        sum += instances;
    }
    println!("{}", sum);
}
