fn main() {
    std::fs::read_to_string("inputs/day_12.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|v| {
                    v.split("-")
                        .map(|v| v.to_string())
                        .map(|v| (v.clone()))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|inputs| {
            inputs.into_iter().fold(
                <std::collections::HashMap<_, _>>::new(),
                |mut cave_system, curr_value| {
                    std::iter::once(0)
                        .map(|_| (&curr_value[0], &curr_value[1]))
                        .map(|(from, to)| {
                            std::iter::once((to != "start").then(|| {
                                cave_system
                                    .entry(from.clone())
                                    .or_insert(vec![])
                                    .push(to.clone())
                            }))
                            .map(|_| {
                                (from != "start").then(|| {
                                    cave_system
                                        .entry(to.clone())
                                        .or_insert(vec![])
                                        .push(from.clone())
                                })
                            })
                            .next()
                            .unwrap()
                        })
                        .next()
                        .map(|_| cave_system)
                        .unwrap()
                },
            )
        })
        .map(|cave_system| {
            (
                cave_system,
                0,
                vec![("start", true, std::collections::HashSet::<&str>::new())],
            )
        })
        .map(|(cave_system, mut no_of_paths, mut stack)| {
            std::iter::repeat(0)
                .map(|_| {
                    stack
                        .pop()
                        .map(
                            |(current_elem, mut can_visit_small_cave_twice, mut visited)| {
                                (current_elem == "end")
                                    .then(|| no_of_paths += 1)
                                    .unwrap_or_else(|| {
                                        (!(visited.contains(&current_elem)
                                            && can_visit_small_cave_twice == false))
                                            .then(|| {
                                                std::iter::once(
                                                    ((&visited).contains(&current_elem)
                                                        && can_visit_small_cave_twice)
                                                        .then(|| {
                                                            can_visit_small_cave_twice = false
                                                        }),
                                                )
                                                .map(|_| {
                                                    (current_elem
                                                        .chars()
                                                        .next()
                                                        .unwrap()
                                                        .is_lowercase())
                                                    .then(|| (&mut visited).insert(current_elem))
                                                })
                                                .map(|_| ())
                                                .next()
                                                .map(|_| {
                                                    cave_system
                                                        .get(current_elem)
                                                        .unwrap()
                                                        .into_iter()
                                                        .map(|ii| {
                                                            stack.push((
                                                                ii.as_str(),
                                                                can_visit_small_cave_twice,
                                                                (&visited).clone(),
                                                            ))
                                                        })
                                                        .last()
                                                        .unwrap_or_default()
                                                })
                                                .unwrap_or_default()
                                            })
                                            .unwrap_or_default()
                                    })
                            },
                        )
                        .map(|_| stack.is_empty())
                        .unwrap()
                })
                .take_while(|v| v == &false)
                .last()
                .map(|_| no_of_paths)
                .unwrap()
        })
        .map(|answer| println!("The answer is {}", answer))
        .unwrap_or_default()
}
