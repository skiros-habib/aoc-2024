mod part1;
mod part2;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn run_p1() {
    assert_eq!(part1::solve(EXAMPLE), 41);
    let result = part1::solve(INPUT);
    println!("Day 6, Part 1: {}", result);
}

pub fn run_p2() {
    assert_eq!(part2::solve(EXAMPLE), 6);
    let result = part2::solve(INPUT);
    println!("Day 6, Part 2: {}", result);
}
