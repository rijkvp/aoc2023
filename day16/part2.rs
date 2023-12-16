mod part1;
use part1::{Dir, Ray};
use std::io::{stdin, BufRead, BufReader};

fn main() {
    let grid = BufReader::new(stdin())
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut max_energized = 0;
    let width = grid[0].len() as i64;
    let height = grid.len() as i64;
    for x in 0..width {
        let count_top = part1::count_energized(
            Ray {
                pos: (x, 0),
                dir: Dir::Down,
            },
            &grid,
        );
        let count_bottom = part1::count_energized(
            Ray {
                pos: (x, height),
                dir: Dir::Up,
            },
            &grid,
        );
        max_energized = max_energized.max(count_top).max(count_bottom);
    }
    for y in 0..height {
        let count_right = part1::count_energized(
            Ray {
                pos: (0, y),
                dir: Dir::Right,
            },
            &grid,
        );
        let count_left = part1::count_energized(
            Ray {
                pos: (width, y),
                dir: Dir::Left,
            },
            &grid,
        );
        max_energized = max_energized.max(count_right).max(count_left);
    }
    println!("{max_energized}");
}
