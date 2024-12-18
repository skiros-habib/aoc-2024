mod part1;
mod part2;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|report| {
            report
                .split_ascii_whitespace()
                .map(|levels| levels.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

pub fn run_p1() {
    assert_eq!(part1::solve(EXAMPLE), 2);
    let result = part1::solve(INPUT);
    println!("Day 2, Part 1: {}", result);
}

pub fn run_p2() {
    assert_eq!(part2::solve(EXAMPLE), 4);
    let result = part2::solve(INPUT);
    println!("Day 2, Part 2: {}", result);
}
