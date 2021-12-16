fn main() {
    std::fs::read_to_string("./inputs/day_15.txt")
        .ok()
        .map(|istr| {
            istr.lines()
                .filter(|v| !v.is_empty())
                .map(|aa| {
                    aa.chars()
                        .map(|v| v as u8 - b'0')
                        .map(|v| v as isize)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|grid| {
            (
                grid.len(),
                grid[0].len(),
                grid,
                [
                    (
                        (|a| a + 1) as fn(usize) -> usize,
                        (|a| a + 0) as fn(usize) -> usize,
                    ),
                    (|a| a + 0, |a| a + 1),
                    (|a: usize| a.wrapping_sub(1), |a| a + 0),
                    (|a| a + 0, |a: usize| a.wrapping_sub(1)),
                ],
                <std::collections::binary_heap::BinaryHeap<(
                    std::cmp::Reverse<isize>,
                    (usize, usize),
                )>>::new(),
            )
        })
        .map(|(ysize, xsize, mut input_grid, dirs, mut queue)| {
            std::iter::successors(
                Some((std::cmp::Reverse(0), (0usize, 0usize))),
                |&(std::cmp::Reverse(current_risk), (xx, yy))| {
                    dirs.map(|(dy, dx)| {
                        std::iter::once(())
                            .map(|_| (dx(xx), dy(yy)))
                            .map(|(neighbour_x, neighbour_y)| {
                                (neighbour_y < ysize
                                    && neighbour_x < xsize
                                    && (input_grid[(neighbour_y)][(neighbour_x)] > 0))
                                    .then(|| {
                                        std::iter::once(0)
                                            .map(|_| {
                                                queue.push((
                                                    std::cmp::Reverse(
                                                        current_risk
                                                            + input_grid[neighbour_y][neighbour_x],
                                                    ),
                                                    (neighbour_x, neighbour_y),
                                                ))
                                            })
                                            .next()
                                            .map(|_| input_grid[neighbour_y][neighbour_x] = 0)
                                            .unwrap()
                                    })
                                    .unwrap_or_default()
                            })
                            .next()
                            .unwrap()
                    })
                    .last()
                    .map(|_| queue.pop())
                    .unwrap()
                },
            )
            .find(|&(_, (xx, yy))| xx == xsize - 1 && yy == ysize - 1)
            .map(|(std::cmp::Reverse(risk), _)| risk)
            .unwrap()
        })
        .map(|answer| println!("The answer is {}", answer))
        .unwrap()
}
