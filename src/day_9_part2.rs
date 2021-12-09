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
                                .map(|v| (v, (xx, yy)))
                                .unwrap()
                        })
                        .collect::<Vec<_>>()
                })
                .map(Some)
                .collect::<Option<Vec<_>>>()
                .map(|v| (ylen, xlen, grid, directions, v))
                .unwrap()
        })
        .map(|(ylen, xlen, grid, directions, unready_low_lands)| {
            unready_low_lands
                .into_iter()
                .flatten()
                .filter(|v| v.0.iter().all(|w| w == &true))
                .map(|v| v.1)
                .map(Some)
                .collect::<Option<Vec<_>>>()
                .map(|v| (ylen, xlen, grid, directions, v))
                .unwrap()
        })
        .map(
            |(ylen, xlen, grid, directions, lowlands): (_, _, _, Vec<(isize, isize)>, _)| {
                lowlands
                    .into_iter()
                    .map(|(l_xx, l_yy)| {
                        std::iter::once(<std::collections::HashSet<(isize, isize)>>::new())
                            .map(|mut hashset| {
                                std::iter::once(vec![(l_xx, l_yy)])
                                    .map(|mut temp_location| {
                                        while !temp_location.is_empty() {
                                            temp_location
                                                .pop()
                                                .map(|(lxx, lyy)| {
                                                    directions
                                                        .clone()
                                                        .into_iter()
                                                        .map(|(aa, bb)| (lxx + aa, lyy + bb))
                                                        .filter(|(xx, yy)| {
                                                            ylen.contains(&yy) && xlen.contains(&xx)
                                                        })
                                                        .filter(|(xx, yy)| {
                                                            grid[*yy as usize][*xx as usize] != 9
                                                        })
                                                        .map(|(xx, yy)| {
                                                            (!(hashset.contains(&(xx, yy))))
                                                                .then(|| {
                                                                    std::iter::once(
                                                                        hashset.insert((xx, yy)),
                                                                    )
                                                                    .next()
                                                                    .map(|_| {
                                                                        temp_location.push((xx, yy))
                                                                    })
                                                                    .unwrap_or_default()
                                                                })
                                                                .unwrap_or_default();
                                                        })
                                                        .last()
                                                        .map(|_| ())
                                                        .unwrap()
                                                })
                                                .unwrap();
                                        }
                                    })
                                    .next()
                                    .map(|_| hashset.len())
                                    .unwrap_or_default()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            },
        )
        .map(|v| {
            v.into_iter()
                .flatten()
                .map(Some)
                .collect::<Option<Vec<_>>>()
                .map(|mut v| {
                    std::iter::once(v.sort_by_key(|w| std::cmp::Reverse(*w)))
                        .map(|_| {
                            println!(
                                "The answer is {}",
                                v.iter().take(3).fold(1, |accum, v| accum * v)
                            )
                        })
                        .next()
                        .map(|_| ())
                        .unwrap_or_default()
                })
                .unwrap_or_default()
        })
        .unwrap_or_default()
}
