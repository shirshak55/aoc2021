fn main() {
    std::fs::read_to_string("./inputs/day_2.txt")
        .ok()
        .map(|v| {
            v.lines()
                .filter(|v| !v.is_empty())
                .map(|v| v.to_string())
                .map(|v| {
                    std::iter::once(v.split(" "))
                        .map(|mut split| {
                            (
                                split.next().unwrap().to_string(),
                                split.next().unwrap().parse::<isize>().unwrap(),
                            )
                        })
                        .next()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .map(|input| (input, (0, 0)))
        .map(|(input, (mut hpos, mut depth))| {
            input
                .into_iter()
                .map(|line| match line.0.as_ref() {
                    "forward" => hpos += line.1,
                    "down" => depth += line.1,
                    "up" => depth -= line.1,
                    _ => unreachable!(),
                })
                .last()
                .map(|_| (hpos, depth))
                .unwrap()
        })
        .map(|(hpos, depth)| println!("The answer is {}", hpos * depth))
        .unwrap_or_default()
}
