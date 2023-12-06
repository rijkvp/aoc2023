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
                .replace(' ', "")
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>();
    let t = nums[0];
    let d = nums[1];
    let mut count = 0;
    for h in 0..t {
        let d2 = h * (t - h);
        if d2 > d {
            count += 1;
        }
    }
    println!("{count}");
}
