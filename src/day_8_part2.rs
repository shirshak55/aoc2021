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
        .map(|v| v[1].iter().map(|v| v.len()).collect::<Vec<_>>())
        .flatten()
        .filter(|v| [2, 4, 3, 7].contains(v))
        .map(Some)
        .collect::<Option<Vec<_>>>()
        .into_iter()
        .map(|v| println!("The answer is {}", v.into_iter().count()))
        .next()
        .unwrap()
}
