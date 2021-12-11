fn main() {
    // day 6 part1 and part2 both are same
    std::fs::read_to_string("inputs/day_6.txt")
        .ok()
        .and_then(|v| {
            v.split(",")
                .map(|v| v.parse::<isize>().ok())
                .collect::<Option<Vec<_>>>()
        })
        .into_iter()
        .map(|v| {
            v.into_iter().fold(
                <std::collections::HashMap<isize, isize>>::new(),
                |mut hm, val| {
                    std::iter::once(*hm.entry(val).or_default() += 1)
                        .next()
                        .map(|_| hm)
                        .unwrap()
                },
            )
        })
        .map(|mut hm| {
            std::iter::once(
                (0..256)
                    .into_iter()
                    .map(|_v| {
                        std::iter::once(*hm.get(&0).unwrap_or(&0))
                            .map(|zero_timed_fish| {
                                (1..=8)
                                    .into_iter()
                                    .map(|v| {
                                        *hm.entry(v - 1).or_default() = *hm.get(&v).unwrap_or(&0)
                                    })
                                    .last()
                                    .map(|_| {
                                        std::iter::once(*hm.entry(8).or_default() = zero_timed_fish)
                                            .map(|_| *hm.entry(6).or_default() += zero_timed_fish)
                                            .last()
                                            .unwrap_or_default()
                                    })
                            })
                            .last()
                            .unwrap_or_default()
                    })
                    .last()
                    .unwrap_or_default(),
            )
            .next()
            .map(|_| hm)
        })
        .map(|v| dbg!(v.unwrap().into_iter().map(|v| v.1).sum::<isize>()))
        .last()
        .map(|_| ())
        .unwrap_or_default()
}
