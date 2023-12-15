use std::io::stdin;

pub fn hash(s: &str) -> u64 {
    let mut val = 0;
    for c in s.chars() {
        val = val + c as u64;
        val *= 17;
        val %= 256;
    }
    val
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let sum = input
        .trim()
        .split(',')
        .into_iter()
        .map(|s| hash(s))
        .sum::<u64>();
    println!("{}", sum);
}
