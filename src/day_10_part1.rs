fn main() {
    std::fs::read_to_string("inputs/day_10.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|v| v.chars().map(|w| w).collect::<Vec<char>>())
                .collect::<Vec<_>>()
        })
        .map(|inputs| {
            (
                inputs,
                std::collections::HashMap::from([('(', ')'), ('<', '>'), ('{', '}'), ('[', ']')]),
                std::collections::HashMap::from([(')', 3), ('}', 1197), (']', 57), ('>', 25137)]),
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
                                        std::iter::once(
                                            matches!(current_char, '(' | '[' | '{' | '<')
                                                .then(|| temp.push(current_char)),
                                        )
                                        .map(|_| None)
                                        .next()
                                        .unwrap(),
                                        (matches!(current_char, ')' | ']' | '}' | '>'))
                                            .then(|| {
                                                ((current_char)
                                                    != (char_map.get(temp.pop().unwrap()).unwrap()))
                                                .then(|| *scores_map.get(current_char).unwrap())
                                            })
                                            .unwrap_or(None),
                                    ][1]
                                })
                                .skip_while(|v| v.is_none())
                                .take(1)
                                // .map(|v| dbg!(v))
                                .next()
                                .unwrap_or(Some(0))
                                .unwrap_or(0)
                        })
                        .next()
                        .unwrap()
                })
                .sum::<isize>()
        })
        .map(|answer| println!("The answer is {answer}", answer = answer))
        .unwrap_or_default()
}
