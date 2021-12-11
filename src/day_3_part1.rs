fn main() {
    std::fs::read_to_string("./inputs/day_3.txt")
        .ok()
        .map(|v| {
            v.lines()
                .filter(|v| !v.is_empty())
                .map(|v| v.trim())
                .map(|v| v.chars().map(|w| w as u8 - b'0').collect::<Vec<u8>>())
                .collect::<Vec<_>>()
        })
        .map(|input| (input.len(), input[0].len(), input))
        .map(|(_length, breadth, input)| {
            (0..breadth)
                .into_iter()
                .map(|offset| {
                    input
                        .clone()
                        .into_iter()
                        .map(|v| v.into_iter())
                        .flatten()
                        .skip(offset)
                        .step_by(breadth)
                        .fold((0, 0), |accum, curr| {
                            (curr == 1)
                                .then(|| (accum.0, accum.1 + 1))
                                .unwrap_or((accum.0 + 1, accum.1))
                        })
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|v| (v.1 >= v.0) as u8)
                .collect::<Vec<_>>()
        })
        .map(|answer_bits| {
            (
                answer_bits
                    .iter()
                    .fold(0isize, |accum, cv| accum * 2 + *cv as isize),
                answer_bits.iter().fold(0isize, |accum, cv| {
                    accum * 2 + (cv == &1).then(|| 0).unwrap_or(1)
                }),
            )
        })
        .map(|answers| println!("The answer is {}", answers.0 * answers.1))
        .unwrap()
}
