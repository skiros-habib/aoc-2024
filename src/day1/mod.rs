mod part1;
mod part2;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

pub fn run_p1() {
    let example = include_str!("example");
    assert_eq!(part1::solve(example), 11);
    let input = include_str!("input");
    let result = part1::solve(input);
    println!("Day 1, Part 1: {}", result);
}

pub fn run_p2() {
    let example = include_str!("example");
    assert_eq!(part2::solve(example), 31);
    let input = include_str!("input");
    let result = part2::solve(input);
    println!("Day 1, Part 2: {}", result);
}
