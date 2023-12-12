mod part1;

fn main() {
    let grid = part1::read_grid();
    let (x, y) = part1::find_start(&grid).unwrap();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    part1::dist_to_farthest((x as isize, y as isize), &grid, &mut visited);
    let mut count = 0;
    for y in 0..grid.len() {
        let mut inside = false;
        for x in 0..grid[0].len() {
            if visited[y][x] {
                let p = grid[y][x];
                if p == '|' || p == 'J' || p == 'L' || p == 'S' {
                    inside = !inside;
                }
                print!("V");
            } else if inside {
                count += 1;
                print!("-");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
    println!("{count}");
}
