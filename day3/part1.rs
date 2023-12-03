use std::io::{stdin, BufRead, BufReader};

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

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

fn is_adjacent(y: usize, x: usize, len: usize, schemetic: &Vec<Vec<char>>) -> bool {
    let row_len = schemetic[0].len();
    for i in x.saturating_sub(1)..(x + len + 1).min(row_len - 1) {
        if (y > 0 && is_symbol(schemetic[y - 1][i]))
            || (y < schemetic.len() - 1 && is_symbol(schemetic[y + 1][i]))
        {
            return true;
        }
    }
    (x > 0 && is_symbol(schemetic[y][x - 1]))
        || (x + len < row_len && is_symbol(schemetic[y][x + len]))
}

fn main() {
    let schemetic: Vec<Vec<char>> = BufReader::new(stdin())
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let mut sum = 0;
    for (y, line) in schemetic.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            let char = line[x];
            if char.is_numeric() {
                let (len, num) = read_num(y, x, &schemetic);
                if is_adjacent(y, x, len, &schemetic) {
                    sum += num;
                }
                x += len;
            } else {
                x += 1;
            }
        }
    }
    println!("{sum}");
}
