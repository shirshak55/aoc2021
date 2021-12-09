fn main() {
    // cargo run --bin day_7
    std::iter::once(
        std::fs::read_to_string("./inputs/day_7.txt")
            .ok()
            .map(|v| {
                v.split(",")
                    .map(|v| v.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
    )
    .next()
    .map(move |input| {
        (
            *input.iter().max().unwrap(),
            *input.iter().min().unwrap(),
            (0isize..input.len() as isize).collect::<Vec<_>>(),
            std::collections::HashMap::new(),
            input,
        )
    })
    .map(|(max, min, mut vv, mut hm, input)| {
        (min..=max)
            .into_iter()
            .map(|ii| {
                input
                    .iter()
                    .enumerate()
                    .map(|jj| {
                        [vv[jj.0 as usize] = (*hm
                            .entry((ii.max(*jj.1), ii.min(*jj.1)))
                            .or_insert(ii.max(*jj.1) - ii.min(*jj.1)))
                            as isize][0]
                    })
                    .last()
                    // for part 1 no need to do sum of series
                    .map(|_| vv.iter().map(|v| v * (v + 1) / 2).sum::<isize>())
                    .unwrap()
            })
            .min()
            .map(|v| println!("The answer is {v}", v = v))
    })
    .map(|_| ())
    .unwrap()
}
