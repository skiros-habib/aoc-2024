use std::collections::HashSet;

mod part1;
mod part2;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

fn basic_parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as usize - 48).collect())
        .collect()
}

fn s_parse(input: &str) -> [HashSet<(usize, usize)>; 10] {
    let grid = basic_parse(input);
    let mut info: [HashSet<(usize, usize)>; 10] = Default::default();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c_set = &mut info[grid[i][j]];
            c_set.insert((i, j));
        }
    }
    info
}

pub fn adjacent_point(x_1: usize, y_1: usize, x_2: usize, y_2: usize) -> bool {
    if x_1 == x_2 && (y_2 as isize - y_1 as isize).abs() == 1 {
        true
    } else if y_1 == y_2 && (x_2 as isize - x_1 as isize).abs() == 1 {
        true
    } else {
        false
    }
}

pub fn run_p1() {
    assert_eq!(part1::solve(EXAMPLE), 36);
    let result = part1::solve(INPUT);
    println!("Day 10, Part 1: {}", result);
}

pub fn run_p2() {
    assert_eq!(part2::solve(EXAMPLE), 81);
    let result = part2::solve(INPUT);
    println!("Day 10, Part 2: {}", result);
}
