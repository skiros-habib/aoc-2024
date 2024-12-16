mod part1;
mod part2;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

pub fn run_p1() {
    assert_eq!(part1::solve(EXAMPLE), 161);
    let result = part1::solve(INPUT);
    println!("Day 3, Part 1: {}", result);
}

pub fn run_p2() {
    assert_eq!(
        part2::solve("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
        48
    );
    let result = part2::solve(INPUT);
    println!("Day 3, Part 2: {}", result);
}
