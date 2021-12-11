fn main() {
    std::fs::read_to_string("./inputs/day_4.txt")
        .ok()
        .map(|inputstr| {
            inputstr
                .lines()
                .filter(|v| !v.is_empty())
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
        })
        .map(|inputs| {
            (
                inputs[0]
                    .split(",")
                    .filter_map(|v| v.parse::<isize>().ok())
                    .collect::<Vec<_>>(),
                inputs
                    .into_iter()
                    .skip(1)
                    .map(|v| {
                        v.split(" ")
                            .filter_map(|v| v.parse::<isize>().ok())
                            .map(|v| (v, false))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
                    .chunks(5)
                    .into_iter()
                    .map(|v| v.to_vec())
                    .collect::<Vec<_>>(),
                <std::vec::Vec<_>>::new(),
            )
        })
        .map(|(drawns, mut boards, mut solved_boards)| {
            drawns
                .into_iter()
                .map(|draw| {
                    std::iter::once(
                        (0..boards.len())
                            .map(|board_id| {
                                (0..boards[0].len())
                                    .map(|ii| {
                                        (0..boards[board_id][ii].len())
                                            .map(|jj| {
                                                (boards[board_id][ii][jj].0 == draw)
                                                    .then(|| boards[board_id][ii][jj].1 = true)
                                                    .unwrap_or_default()
                                            })
                                            .last()
                                            .unwrap_or_default()
                                    })
                                    .last()
                                    .unwrap_or_default()
                            })
                            .last()
                            .unwrap_or_default(),
                    )
                    .map(|_| {
                        (0..boards.len())
                            .map(|board_id| {
                                std::iter::once((!solved_boards.contains(&board_id)).then(|| {
                                    (0..boards[board_id].len())
                                        .map(|ii| {
                                            (boards[board_id][ii].iter().filter(|v| v.1).count()
                                                == boards[board_id][ii].len())
                                            .then(|| solved_boards.push(board_id))
                                        })
                                        .last()
                                        .unwrap_or_default()
                                }))
                                .map(|_| {
                                    (!solved_boards.contains(&board_id)).then(|| {
                                        // col check
                                        (0..boards[board_id][0].len())
                                            .map(|column_selector| {
                                                (boards[board_id]
                                                    .iter()
                                                    .map(|row| row[column_selector])
                                                    .filter(|v| v.1)
                                                    .count()
                                                    == boards[board_id][0].len())
                                                .then(|| solved_boards.push(board_id))
                                            })
                                            .last()
                                            .unwrap_or_default()
                                    })
                                })
                                .next()
                                .unwrap_or_default()
                            })
                            .last()
                            .map(|_| {
                                draw * (solved_boards.len() == boards.len())
                                    .then(|| {
                                        boards[solved_boards[solved_boards.len() - 1 as usize]]
                                            .iter()
                                            .map(|v| {
                                                v.iter()
                                                    .filter(|v| v.1 == false)
                                                    .map(|v| v.0)
                                                    .sum::<isize>()
                                            })
                                            .sum::<isize>()
                                    })
                                    .unwrap_or(0)
                            })
                            .unwrap_or_default()
                    })
                    .next()
                    .unwrap()
                })
                .find(|v| v > &0)
                .unwrap()
        })
        .map(|answer| println!("The answer is {}", answer))
        .unwrap()
}
