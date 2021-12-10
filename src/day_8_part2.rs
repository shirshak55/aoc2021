fn main() {
    std::fs::read_to_string("./inputs/day_8.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|v| v.split_once(" | ").unwrap())
                .map(|v| {
                    [v.0, v.1]
                        .into_iter()
                        .map(|v| {
                            v.split(" ")
                                .map(|v| v.trim().to_string())
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .unwrap()
        .into_iter()
        .map(|input| {
            (
                input,
                std::collections::HashMap::from([
                    (42, 0), // 0 is a + b + c + f + e + g
                    (17, 1), // 1 is two letter c and f only
                    (34, 2),
                    (39, 3),
                    (30, 4),
                    (37, 5),
                    (41, 6),
                    (25, 7),
                    (49, 8),
                    (45, 9),
                ]),
            )
        })
        .map(|(input, number_maps)| {
            std::iter::once(
                "abcdefg"
                    .chars()
                    // ichar= input char
                    .map(|ichar| {
                        (
                            ichar,
                            input[0]
                                .iter()
                                .filter(|&input| input.contains(ichar))
                                .count() as isize,
                        )
                    })
                    .collect::<std::collections::HashMap<_, _>>(),
            )
            .map(|hm| {
                input[1]
                    .iter()
                    .map(|one_input| {
                        one_input
                            .chars()
                            .map(|cch| *hm.get(&cch).unwrap())
                            .sum::<isize>()
                    })
                    .map(|num| number_maps.get(&num).unwrap())
                    .fold(0, |accum, value| accum * 10 + value)
            })
            .next()
            .unwrap()
        })
        .map(Some)
        .collect::<Option<Vec<_>>>()
        .into_iter()
        .map(|outputs| {
            println!(
                "The answer is {answer}",
                answer = outputs.into_iter().sum::<isize>()
            )
        })
        .last()
        .unwrap()
}
