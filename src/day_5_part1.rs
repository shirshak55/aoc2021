fn main() {
    std::fs::read_to_string("./inputs/day_5.txt")
        .ok()
        .map(|input| {
            input
                .lines()
                .map(|v| {
                    v.split("->")
                        .map(|arrow_s| {
                            arrow_s
                                .split(",")
                                .into_iter()
                                .map(|comma_s| comma_s.trim().parse::<isize>().unwrap())
                                .into_iter()
                        })
                        .flatten()
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|input| {
            (
                input
                    .into_iter()
                    .filter(|v| v[0] == v[2] || v[1] == v[3])
                    .collect::<Vec<_>>(),
                <std::collections::HashMap<(isize, isize), isize>>::new(),
            )
        })
        .map(|(input, mut map)| {
            input
                .into_iter()
                .map(|line| {
                    (line[0].min(line[2])..=line[0].max(line[2]))
                        .map(|x_coord| {
                            (line[1].min(line[3])..=line[1].max(line[3]))
                                .map(|y_coord| {
                                    *map.entry((x_coord, y_coord)).or_insert(0) += 1;
                                })
                                .last()
                                .unwrap();
                        })
                        .last()
                        .unwrap();
                })
                .last()
                .map(|_| map)
                .unwrap()
        })
        .map(|v| v.into_values().filter(|v| *v > 1).count())
        .map(|answer| println!("The answer is {answer}", answer = answer))
        .unwrap();
}
