fn main() {
    std::fs::read_to_string("./inputs/day_14.txt")
        .ok()
        .map(|v| {
            (
                v.lines().next().unwrap().to_string(),
                v.lines()
                    .next()
                    .unwrap()
                    .chars()
                    .collect::<Vec<_>>()
                    .windows(2)
                    .map(|v| v.iter().collect::<String>())
                    .collect::<std::vec::Vec<_>>(),
                v.lines()
                    .skip(2)
                    .map(|v| v.split(" -> ").map(|v| v.to_owned()).collect::<Vec<_>>())
                    .map(|v| (v.get(0).unwrap().to_owned(), v.get(1).unwrap().to_owned()))
                    .collect::<std::collections::HashMap<_, _>>(),
            )
        })
        .map(|(polymer_str, v, rules)| {
            (
                polymer_str,
                v.into_iter().fold(
                    <std::collections::HashMap<String, isize>>::new(),
                    |mut accum, curr| {
                        std::iter::once(*accum.entry(curr).or_insert(0) += 1)
                            .next()
                            .map(|_| accum)
                            .unwrap()
                    },
                ),
                rules,
            )
        })
        .map(|(polymer_str, mut polymers, rules)| {
            (1..=10) // for part to change 10 to 40
                .into_iter()
                .map(|_| std::collections::HashMap::<String, isize>::new())
                .map(|mut new_polymers| {
                    polymers
                        .iter()
                        .map(|(polymer_name, pcount)| {
                            rules
                                .get(polymer_name)
                                .map(|char_to_insert| {
                                    (
                                        format!(
                                            "{}{}",
                                            polymer_name.chars().next().unwrap(),
                                            char_to_insert
                                        ),
                                        format!(
                                            "{}{}",
                                            char_to_insert,
                                            polymer_name.chars().skip(1).next().unwrap()
                                        ),
                                    )
                                })
                                .map(|(first, last)| {
                                    std::iter::once(
                                        *new_polymers.entry(first).or_insert(0) += pcount,
                                    )
                                    .next()
                                    .map(|_| *new_polymers.entry(last).or_insert(0) += pcount)
                                    .unwrap_or_default()
                                })
                                .unwrap_or_default()
                        })
                        .last()
                        .map(|_| polymers = new_polymers)
                        .unwrap_or_default()
                })
                .last()
                .map(|_| {
                    polymers.into_iter().fold(
                        std::collections::HashMap::<char, isize>::new(),
                        |mut accum, record| {
                            std::iter::once(
                                record
                                    .0
                                    .chars()
                                    .next()
                                    .map(|current| *accum.entry(current).or_default() += record.1),
                            )
                            .next()
                            .map(|_| accum)
                            .unwrap_or_default()
                        },
                    )
                })
                .map(|mut counts| {
                    std::iter::once(
                        *counts
                            .entry(polymer_str.chars().last().unwrap())
                            .or_default() += 1,
                    )
                    .next()
                    .map(|_| counts.values().max().unwrap() - counts.values().min().unwrap())
                    .unwrap_or_default()
                })
                .unwrap()
        })
        .map(|answer| println!("The answer is {}", answer))
        .unwrap_or_default()
}
