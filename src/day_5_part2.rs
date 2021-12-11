fn main() {
    std::iter::once((|input: String| {
        input
            .lines()
            .map(|v| v.trim())
            .flat_map(|v| v.split("->"))
            .flat_map(|v| v.split(","))
            .map(|v| v.trim().parse::<isize>().unwrap())
            .map(Some)
            .collect::<Option<Vec<_>>>()
            .into_iter()
            .flat_map(move |v| {
                v.chunks(4)
                    .map(|v| v.to_vec())
                    .map(|v| (v[0], v[1], v[2], v[3], v[0].cmp(&v[2]), v[1].cmp(&v[3])))
                    .collect::<Vec<_>>()
            })
            .map(|w| {
                matches!(w.4, std::cmp::Ordering::Equal)
                    .then(|| {
                        (std::cmp::min(w.1, w.3)..=std::cmp::max(w.1, w.3))
                            .into_iter()
                            .map(|val| (w.0, val))
                            .into_iter()
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_else(|| {
                        matches!(w.5, std::cmp::Ordering::Equal)
                            .then(|| {
                                (std::cmp::min(w.0, w.2)..=std::cmp::max(w.0, w.2))
                                    .into_iter()
                                    .map(|val| (val, w.1))
                                    .into_iter()
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_else(|| {
                                // remove this block and use vec![] for part1
                                (0..)
                                    .into_iter()
                                    .map(|v| {
                                        (
                                            w.0 - v * (w.0 - w.2) / (w.0 - w.2).abs(),
                                            w.1 - v * (w.1 - w.3) / (w.1 - w.3).abs(),
                                        )
                                    })
                                    .map(|v| (v))
                                    .take_while(|z| (z.0) != (w.2))
                                    .chain(std::iter::once((w.2, w.3)))
                                    .collect::<Vec<_>>()
                            })
                    })
            })
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
            .map(Some)
            .collect::<Option<Vec<_>>>()
            .into_iter()
            .map(|vec| {
                vec.into_iter()
                    .fold(
                        std::collections::HashMap::<(isize, isize), isize>::new(),
                        |mut hm, value| {
                            std::iter::once(*hm.entry(value).or_default() += 1)
                                .next()
                                .map(|_| hm)
                                .unwrap()
                        },
                    )
                    .iter()
                    .filter(|v| v.1 > &1)
                    .count()
            })
            .map(|v| println!("The answer is {}", v))
            .map(drop)
            .last()
            .unwrap()
    })(include_str!("../inputs/day_5.txt").to_string()))
    .next()
    .map(|_| ())
    .unwrap()
}
