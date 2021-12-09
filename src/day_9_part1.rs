fn main() {
    std::fs::read_to_string("inputs/day_9.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|v| {
                    v.chars()
                        .map(|w| w as isize - '0' as isize)
                        .collect::<Vec<isize>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|v| {
            (
                0isize..v.len() as isize,
                0isize..v[0].len() as isize,
                v,
                vec![(-1, 0), (0, -1), (0, 1), (1, 0)],
            )
        })
        .map(|(ylen, xlen, grid, directions)| {
            ylen.clone()
                .map(|yy| {
                    (xlen.clone())
                        .map(|xx| {
                            directions
                                .clone()
                                .into_iter()
                                .map(|kk| {
                                    (ylen.contains(&(yy + kk.1))
                                        && xlen.contains(&(xx + kk.0))
                                        && grid[(yy) as usize][(xx) as usize]
                                            >= grid[(yy + kk.1) as usize][(xx + kk.0) as usize])
                                        .then(|| false)
                                        .unwrap_or(true)
                                })
                                .map(Some)
                                .collect::<Option<Vec<_>>>()
                                .map(|v| (v, grid[(yy) as usize][(xx) as usize] + 1))
                                .unwrap()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|v| {
            v.into_iter()
                .flatten()
                .filter(|v| v.0.iter().all(|w| w == &true))
                .map(|v| v.1)
                .sum::<isize>()
        })
        .map(|v| println!("The answer is {v}", v = v))
        .unwrap_or_default()
}
