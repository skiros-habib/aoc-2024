mod part1;
mod part2;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

pub fn run_p1() {
    assert_eq!(part1::solve(EXAMPLE), 1928);
    let result = part1::solve(INPUT);
    println!("Day 10, Part 1: {}", result);
}

pub fn run_p2() {
    assert_eq!(part2::solve(EXAMPLE), 2858);
    let result = part2::solve(INPUT);
    println!("Day 10, Part 2: {}", result);
}
