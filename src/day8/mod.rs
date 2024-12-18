use std::collections::{BTreeSet, HashMap};
mod part1;
mod part2;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

fn parse(input: &str) -> (HashMap<char, BTreeSet<(usize, usize)>>, (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut info: HashMap<char, BTreeSet<(usize, usize)>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j].is_alphanumeric() {
                let c_vec = info.entry(grid[i][j]).or_insert(BTreeSet::new());
                c_vec.insert((i, j));
            }
        }
    }
    (info, (grid.len(), grid[0].len()))
}

pub fn run_p1() {
    assert_eq!(part1::solve(EXAMPLE), 14);
    let result = part1::solve(INPUT);
    println!("Day 8, Part 1: {}", result);
}

pub fn run_p2() {
    assert_eq!(part2::solve(EXAMPLE), 34);
    let result = part2::solve(INPUT);
    println!("Day 8, Part 2: {}", result);
}
