fn main() {
    std::fs::read_to_string("inputs/day_11.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|v| v.bytes().map(|w| w - b'0').collect::<Vec<u8>>())
                .collect::<Vec<_>>()
        })
        .map(|grid| {
            (
                [
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                ],
                0isize..grid.len() as isize,
                0isize..grid[0].len() as isize,
                grid,
                0,
                <std::collections::VecDeque<(isize, isize)>>::new(),
            )
        })
        .map(|(dirs, yrange, xrange, mut grid, mut flashes, mut queue)| {
            std::iter::repeat(())
                .take(100)
                .map(|_| {
                    yrange
                        .clone()
                        .map(|yy| {
                            xrange
                                .clone()
                                .map(|xx| {
                                    (grid[yy as usize][xx as usize] == 9)
                                        .then(|| {
                                            std::iter::once(flashes += 1)
                                                .map(|_| grid[yy as usize][xx as usize] = 0)
                                                .map(|_| queue.push_back((xx, yy)))
                                                .last()
                                                .unwrap_or_default()
                                        })
                                        .unwrap_or_else(|| grid[yy as usize][xx as usize] += 1)
                                })
                                .last()
                                .unwrap_or_default()
                        })
                        .last()
                        .map(|_| {
                            std::iter::repeat(0)
                                .map(|_| {
                                    queue
                                        .pop_front()
                                        .map(|octopus| {
                                            dirs.map(|dneigh| {
                                                std::iter::once((
                                                    dneigh.0 + octopus.0,
                                                    dneigh.1 + octopus.1,
                                                ))
                                                .map(|(dx, dy)| {
                                                    (yrange.clone().contains(&dy)
                                                        && xrange.clone().contains(&dx))
                                                    .then(|| match grid[dy as usize][dx as usize] {
                                                        0 => {}
                                                        9 => std::iter::once(flashes += 1)
                                                            .map(|_| {
                                                                grid[dy as usize][dx as usize] = 0
                                                            })
                                                            .map(|_| queue.push_back((dx, dy)))
                                                            .next()
                                                            .unwrap_or_default(),
                                                        _ => grid[dy as usize][dx as usize] += 1,
                                                    })
                                                })
                                                .last()
                                                .unwrap_or_default()
                                            })
                                        })
                                        .is_none()
                                })
                                .take_while(|is_none| is_none == &false)
                                .last()
                                .unwrap_or_default()
                        })
                        .unwrap_or_default()
                })
                .last()
                .map(|_| flashes)
                .unwrap()
        })
        .map(|answer| println!("The answer is {answer}", answer = answer))
        .unwrap()
}
