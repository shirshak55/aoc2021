fn main() {
    std::fs::read_to_string("./inputs/day_1.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|a| a.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|input| input.windows(2).filter(|v| v[1] > v[0]).count())
        .map(|answer| println!("The answer is {}", answer))
        .unwrap();
}
