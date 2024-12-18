mod part1;
mod part2;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    let mut res = Vec::new();
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .expect("An equation consists of test_value`: `numbers")
        })
        .for_each(|(test_value, numbers)| {
            let test_value = test_value.parse().expect("test_value must be a number");
            let numbers = numbers
                .split(" ")
                .map(|n| n.parse().expect("numbers must be numbers"))
                .collect();
            res.push((test_value, numbers));
        });
    res
}

pub fn run_p1() {
    assert_eq!(part1::solve(EXAMPLE), 3749);
    let result = part1::solve(INPUT);
    println!("Day 7, Part 1: {}", result);
}

pub fn run_p2() {
    assert_eq!(part2::solve(EXAMPLE), 11387);
    let result = part2::solve(INPUT);
    println!("Day 7, Part 2: {}", result);
}
