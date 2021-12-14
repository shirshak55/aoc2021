fn main() {
    std::fs::read_to_string("./inputs/day_13.txt")
        .ok()
        .map(|istr| {
            (
                istr.lines()
                    .filter(|v| !v.is_empty())
                    .filter(|v| v.starts_with("fold along"))
                    .map(|v| v.replace("fold along ", ""))
                    .map(|v| v.split("=").map(|v| v.to_owned()).collect::<Vec<_>>())
                    .map(|v| {
                        (
                            v.get(0).unwrap().to_string(),
                            v.get(1).unwrap().parse::<isize>().unwrap(),
                        )
                    })
                    .collect::<Vec<_>>(),
                istr.lines()
                    .filter(|v| !v.is_empty())
                    .filter(|v| !v.starts_with("fold along"))
                    .map(|v| {
                        v.split(",")
                            .map(|v| v.parse::<isize>())
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .collect::<Result<std::collections::BTreeSet<_>, _>>()
                    .unwrap(),
            )
        })
        .map(|(folding_rules, mut dots)| {
            folding_rules
                .into_iter()
                .map(|(fold_dir, fold_quantity)| {
                    (fold_dir, fold_quantity, std::collections::BTreeSet::new())
                })
                .map(|(fold_dir, fold_quantity, mut new_dots)| {
                    std::iter::once(1)
                        .map(|_| {
                            (fold_dir == "y")
                                .then(|| {
                                    for dd in dots.iter() {
                                        (&dd[1] != &fold_quantity)
                                            .then(|| {
                                                (&dd[1] > &fold_quantity)
                                                    .then(|| {
                                                        new_dots
                                                            .insert(vec![
                                                                dd[0],
                                                                fold_quantity
                                                                    - (dd[1] - fold_quantity),
                                                            ])
                                                            .then(|| {})
                                                            .unwrap_or_default()
                                                    })
                                                    .unwrap_or_else(|| {
                                                        new_dots
                                                            .insert(dd.clone())
                                                            .then(|| {})
                                                            .unwrap_or_default()
                                                    })
                                            })
                                            .unwrap_or_default()
                                    }
                                })
                                .unwrap_or_else(|| {
                                    for dd in dots.iter() {
                                        (&dd[0] != &fold_quantity)
                                            .then(|| {
                                                (dd[0] > fold_quantity)
                                                    .then(|| {
                                                        new_dots
                                                            .insert(vec![
                                                                fold_quantity
                                                                    - (dd[0] - fold_quantity),
                                                                dd[1],
                                                            ])
                                                            .then(|| {})
                                                            .unwrap_or_default()
                                                    })
                                                    .unwrap_or_else(|| {
                                                        new_dots
                                                            .insert(dd.clone())
                                                            .then(|| {})
                                                            .unwrap_or_default()
                                                    })
                                            })
                                            .unwrap_or_default()
                                    }
                                })
                        })
                        .next()
                        .map(|_| dots = new_dots)
                        .unwrap()
                })
                .last() // change this to next for part 1 :D
                .map(|_| dots)
                .unwrap()
        })
        .map(|dots| {
            (
                dots,
                // vec macro not allowed because of semicolons
                // poor's man way to initialize vec without semicolon
                (0..10)
                    .into_iter()
                    .map(|_| (0..50).into_iter().map(|_| "⬜").collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(dots, mut grid)| {
            std::iter::once(
                dots.iter()
                    .for_each(|dd| grid[dd[1] as usize][dd[0] as usize] = "⬛"),
            )
            .map(|_| {
                grid.iter().for_each(|ii| {
                    println!(
                        "{}",
                        ii.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join("")
                    )
                })
            })
            .next()
            .unwrap_or_default()
        })
        .unwrap()
}
