use std::io::{stdin, BufRead, BufReader};

#[derive(Debug)]
pub struct Ray {
    pub pos: (i64, i64),
    pub dir: Dir,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn apply(&self, pos: (i64, i64)) -> (i64, i64) {
        match self {
            Dir::Up => (pos.0, pos.1 - 1),
            Dir::Down => (pos.0, pos.1 + 1),
            Dir::Left => (pos.0 - 1, pos.1),
            Dir::Right => (pos.0 + 1, pos.1),
        }
    }

    fn next(self, c: char) -> Vec<Dir> {
        match c {
            '.' => vec![self],
            '/' => vec![match self {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Up,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Left,
            }],
            '\\' => vec![match self {
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Up,
                Dir::Down => Dir::Right,
                Dir::Right => Dir::Down,
            }],
            '|' => match self {
                Dir::Left | Dir::Right => vec![Dir::Up, Dir::Down],
                Dir::Up | Dir::Down => vec![self],
            },
            '-' => match self {
                Dir::Up | Dir::Down => vec![Dir::Left, Dir::Right],
                Dir::Left | Dir::Right => vec![self],
            },
            _ => unreachable!("unexpected character"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Visited {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl Visited {
    fn any(&self) -> bool {
        self.left || self.right || self.up || self.down
    }
    fn dir(&self, dir: Dir) -> bool {
        match dir {
            Dir::Up => self.up,
            Dir::Down => self.down,
            Dir::Left => self.left,
            Dir::Right => self.right,
        }
    }
}

pub fn count_energized(start_ray: Ray, grid: &Vec<Vec<char>>) -> u64 {
    let height = grid.len();
    let width = grid[0].len();
    let mut rays = vec![start_ray];
    let mut energized = vec![vec![Visited::default(); width]; height];
    while let Some(mut ray) = rays.pop() {
        while ray.pos.0 >= 0
            && ray.pos.0 < width as i64
            && ray.pos.1 >= 0
            && ray.pos.1 < height as i64
        {
            if energized[ray.pos.1 as usize][ray.pos.0 as usize].dir(ray.dir) {
                break;
            }
            let (x, y) = (ray.pos.0 as usize, ray.pos.1 as usize);
            match ray.dir {
                Dir::Up => energized[y][x].up = true,
                Dir::Down => energized[y][x].down = true,
                Dir::Left => energized[y][x].left = true,
                Dir::Right => energized[y][x].right = true,
            };
            let c = grid[y][x];
            let dirs = ray.dir.next(c);
            if dirs.len() > 1 {
                rays.push(Ray {
                    pos: dirs[1].apply(ray.pos),
                    dir: dirs[1],
                });
            }
            ray.pos = dirs[0].apply(ray.pos);
            ray.dir = dirs[0];
        }
    }
    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if energized[y][x].any() {
                sum += 1;
            }
        }
    }
    sum
}

fn main() {
    let grid = BufReader::new(stdin())
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let sum = count_energized(
        Ray {
            pos: (0, 0),
            dir: Dir::Right,
        },
        &grid,
    );
    println!("{sum}");
}
