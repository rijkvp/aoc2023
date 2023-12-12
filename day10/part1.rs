use std::io::stdin;
use std::io::BufRead;

fn find_next(
    (prev_x, prev_y): (isize, isize),
    (x, y): (isize, isize),
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> u64 {
    if x < 0 || x as usize > grid[0].len() || y < 0 || y as usize > grid.len() {
        return 0;
    }
    if visited[y as usize][x as usize] {
        return 0;
    }
    let dx = x - prev_x;
    let dy = y - prev_y;
    return 1 + match grid[y as usize][x as usize] {
        '.' => 0,
        '-' => {
            if dx.abs() == 1 {
                visited[y as usize][x as usize] = true;
                find_next((x, y), (x + dx, y), grid, visited)
            } else {
                0
            }
        }
        '|' => {
            if dy.abs() == 1 {
                visited[y as usize][x as usize] = true;
                find_next((x, y), (x, y + dy), grid, visited)
            } else {
                0
            }
        }
        'L' => {
            if dy == 1 {
                visited[y as usize][x as usize] = true;
                find_next((x, y), (x + 1, y), grid, visited)
            } else if dx == -1 {
                visited[y as usize][x as usize] = true;
                find_next((x, y), (x, y - 1), grid, visited)
            } else {
                0
            }
        }
        'J' => {
            if dy == 1 {
                visited[y as usize][x as usize] = true;
                find_next((x, y), (x - 1, y), grid, visited)
            } else if dx == 1 {
                visited[y as usize][x as usize] = true;
                find_next((x, y), (x, y - 1), grid, visited)
            } else {
                0
            }
        }
        '7' => {
            if dy == -1 {
                visited[y as usize][x as usize] = true;
                find_next((x, y), (x - 1, y), grid, visited)
            } else if dx == 1 {
                visited[y as usize][x as usize] = true;
                find_next((x, y), (x, y + 1), grid, visited)
            } else {
                0
            }
        }
        'F' => {
            if dy == -1 {
                visited[y as usize][x as usize] = true;
                find_next((x, y), (x + 1, y), grid, visited)
            } else if dx == -1 {
                visited[y as usize][x as usize] = true;
                find_next((x, y), (x, y + 1), grid, visited)
            } else {
                0
            }
        }
        'S' => {
            visited[y as usize][x as usize] = true;
            0
        }
        _ => unreachable!(),
    };
}

pub fn dist_to_farthest(
    (start_x, start_y): (isize, isize),
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> u64 {
    find_next((start_x, start_y), (start_x - 1, start_y), &grid, visited)
        .max(find_next(
            (start_x, start_y),
            (start_x + 1, start_y),
            &grid,
            visited,
        ))
        .max(find_next(
            (start_x, start_y),
            (start_x, start_y + 1),
            &grid,
            visited,
        ))
        .max(find_next(
            (start_x, start_y),
            (start_x, start_y - 1),
            &grid,
            visited,
        ))
        / 2
}

pub fn read_grid() -> Vec<Vec<char>> {
    stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect()
}

pub fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}

fn main() {
    let grid = read_grid();
    let (x, y) = find_start(&grid).unwrap();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    println!(
        "{}",
        dist_to_farthest((x as isize, y as isize), &grid, &mut visited)
    );
}
