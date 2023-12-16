use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut grid: Vec<Vec<char>> = BufReader::new(stdin())
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let width = grid[0].len();
    let height = grid.len();
    for y in 1..height {
        for x in 0..width {
            if grid[y][x] == 'O' {
                let mut move_y = y;
                while move_y != 0 && grid[move_y - 1][x] == '.' {
                    move_y -= 1;
                }
                grid[y][x] = '.';
                grid[move_y][x] = 'O';
            }
        }
    }
    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            let load = height - y;
            if grid[y][x] == 'O' {
                sum += load;
            }
        }
    }
    println!("{sum}");
}
