use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

fn read_num(y: usize, x: usize, schemetic: &Vec<Vec<char>>) -> (usize, u64) {
    let mut string = String::new();
    let mut char = schemetic[y][x];
    let mut len = 0;
    let row_len = schemetic[0].len();
    while char.is_numeric() {
        string.push(char);
        len += 1;
        if x + len >= row_len {
            break;
        }
        char = schemetic[y][x + len];
    }
    (len, string.parse().unwrap())
}

fn find_gears(
    y: usize,
    x: usize,
    len: usize,
    num: u64,
    schemetic: &Vec<Vec<char>>,
    gears: &mut HashMap<(usize, usize), Vec<u64>>,
) {
    let row_len = schemetic[0].len();
    for i in x.saturating_sub(1)..(x + len + 1).min(row_len - 1) {
        if y > 0 && schemetic[y - 1][i] == '*' {
            gears.entry((y - 1, i)).or_insert_with(Vec::new).push(num);
        }
        if y < schemetic.len() - 1 && schemetic[y + 1][i] == '*' {
            gears.entry((y + 1, i)).or_insert_with(Vec::new).push(num);
        }
    }
    if x > 0 && schemetic[y][x - 1] == '*' {
        gears.entry((y, x - 1)).or_insert_with(Vec::new).push(num);
    }
    if x + len < row_len && schemetic[y][x + len] == '*' {
        gears.entry((y, x + len)).or_insert_with(Vec::new).push(num);
    }
}

fn main() {
    let schemetic: Vec<Vec<char>> = BufReader::new(stdin())
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let mut sum = 0;
    let mut gears = HashMap::new();
    for (y, line) in schemetic.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            let char = line[x];
            if char.is_numeric() {
                let (len, num) = read_num(y, x, &schemetic);
                find_gears(y, x, len, num, &schemetic, &mut gears);
                x += len;
            } else {
                x += 1;
            }
        }
    }
    for nums in gears.values() {
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }
    println!("{sum}")
}
