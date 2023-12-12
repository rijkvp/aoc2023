mod part1;

fn main() {
    let input = part1::read_input();
    let count: u64 = input
        .into_iter()
        .map(|(mut s, g)| {
            s.push('?');
            let s_len = s.len();
            let g_len = g.len();
            (
                s.into_iter()
                    .cycle()
                    .take(s_len * 5 - 1)
                    .collect::<Vec<_>>(),
                g.into_iter().cycle().take(g_len * 5).collect::<Vec<_>>(),
            )
        })
        .map(|(mut s, g)| part1::count_arrangements(&mut s, &g))
        .sum();
    println!("{count}")
}
