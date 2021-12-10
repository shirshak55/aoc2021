fn main() {
    std::fs::read_to_string("inputs/day_10.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|v| v.chars().map(|w| w).collect::<Vec<char>>())
                .collect::<Vec<_>>()
        })
        .map(|v| {
            (
                v,
                std::collections::HashMap::from([('(', ')'), ('<', '>'), ('{', '}'), ('[', ']')]),
                std::collections::HashMap::from([('(', 1), ('{', 3), ('[', 2), ('<', 4)]),
            )
        })
        .map(|(inputs, char_map, scores_map)| {
            inputs
                .iter()
                .map(|line| {
                    std::iter::once(vec![])
                        .map(|mut temp| {
                            line.iter()
                                .map(|current_char| {
                                    [
                                        matches!(current_char, '(' | '[' | '{' | '<')
                                            .then(|| (temp.push(current_char))),
                                        (matches!(current_char, ')' | ']' | '}' | '>'))
                                            .then(|| {
                                                (current_char
                                                    == char_map.get(temp.pop().unwrap()).unwrap())
                                                .then(|| ())
                                            })
                                            .unwrap_or(Some(())),
                                    ][1]
                                })
                                .collect::<Option<Vec<_>>>()
                                .map(|_| {
                                    std::iter::once(temp.reverse())
                                        .map(|_| {
                                            temp.iter()
                                                .map(|v| *scores_map.get(v).unwrap())
                                                .fold(0isize, |accum, value| accum * 5 + value)
                                        })
                                        .next()
                                        .unwrap()
                                })
                        })
                        .next()
                        .unwrap()
                })
                .filter(Option::is_some)
                .collect::<Option<Vec<_>>>()
                .map(|mut vec_of_potential_ans| {
                    std::iter::once(vec_of_potential_ans.sort())
                        .map(|_| vec_of_potential_ans[(vec_of_potential_ans.len() - 1) / 2])
                        .next()
                        .unwrap()
                })
                .map(|v| println!("The answer is {}", v))
                .unwrap_or_default()
        })
        .unwrap_or_default()
}
