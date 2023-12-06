use std::io::{stdin, BufRead, BufReader};

fn main() {
    let lines = BufReader::new(stdin())
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let nums = lines
        .into_iter()
        .map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let mut product = 1;
    for (t, d) in nums[0].iter().zip(nums[1].iter()) {
        let mut count = 0;
        for h in 0..*t {
            let d2 = h * (t - h);
            if d2 > *d {
                count += 1;
            }
        }
        product *= count;
    }
    println!("{product}");
}
