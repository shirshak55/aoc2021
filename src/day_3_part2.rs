fn main() {
    std::fs::read_to_string("./inputs/day_3.txt")
        .ok()
        .map(|v| {
            v.lines()
                .filter(|v| !v.is_empty())
                .map(|v| v.trim())
                .map(|v| v.chars().map(|w| w as u8 - b'0').collect::<Vec<u8>>())
                .collect::<Vec<_>>()
        })
        .map(|input| (input[0].len(), input.clone(), input))
        .map(|(breadth, mut o2, mut co2)| {
            (0..breadth)
                .into_iter()
                .map(|ii| {
                    if o2.len() > 1 {
                        let mut unknown_a = o2.clone();
                        let mut unknown_b = o2.clone();

                        unknown_a.retain(|vv| vv[ii] == 1);
                        unknown_b.retain(|vv| vv[ii] == 0);

                        if unknown_a.len() >= unknown_b.len() {
                            o2 = unknown_a;
                        } else {
                            o2 = unknown_b;
                        }
                    }
                    if co2.len() > 1 {
                        let mut unknown_c = co2.clone();
                        let mut unknown_d = co2.clone();
                        unknown_c.retain(|vv| vv[ii] == 0);
                        unknown_d.retain(|vv| vv[ii] == 1);

                        if unknown_c.len() <= unknown_d.len() {
                            *&mut co2 = unknown_c;
                        } else {
                            *&mut co2 = unknown_d;
                        }
                    }
                })
                .last()
                .map(|_| {
                    [o2, co2].map(|v| {
                        v[0].iter()
                            .fold(0isize, |accum, val| accum * 2 + *val as isize)
                    })
                })
                .map(|v| v[0] * v[1])
                .unwrap_or_default()
        })
        .map(|answer| println!("The answer is {answer}", answer = answer))
        .unwrap();
}
