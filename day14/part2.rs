use std::io::{stdin, BufRead, BufReader};

fn tilt_north(grid: &mut Vec<Vec<char>>, width: usize, height: usize) {
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
}

fn tilt_south(grid: &mut Vec<Vec<char>>, width: usize, height: usize) {
    for y in (0..height - 1).rev() {
        for x in 0..width {
            if grid[y][x] == 'O' {
                let mut move_y = y;
                while move_y < height - 1 && grid[move_y + 1][x] == '.' {
                    move_y += 1;
                }
                grid[y][x] = '.';
                grid[move_y][x] = 'O';
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>, width: usize, height: usize) {
    for x in 1..width {
        for y in 0..height {
            if grid[y][x] == 'O' {
                let mut move_x = x;
                while move_x != 0 && grid[y][move_x - 1] == '.' {
                    move_x -= 1;
                }
                grid[y][x] = '.';
                grid[y][move_x] = 'O';
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>, width: usize, height: usize) {
    for x in (0..width - 1).rev() {
        for y in 0..height {
            if grid[y][x] == 'O' {
                let mut move_x = x;
                while move_x < width - 1 && grid[y][move_x + 1] == '.' {
                    move_x += 1;
                }
                grid[y][x] = '.';
                grid[y][move_x] = 'O';
            }
        }
    }
}

fn main() {
    let mut grid: Vec<Vec<char>> = BufReader::new(stdin())
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let width = grid[0].len();
    let height = grid.len();
    // Somehow it works with exactly 1000 iterations :-)
    for _ in 0..1000 {
        tilt_north(&mut grid, width, height);
        tilt_west(&mut grid, width, height);
        tilt_south(&mut grid, width, height);
        tilt_east(&mut grid, width, height);
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
