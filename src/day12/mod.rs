use std::collections::{HashMap, HashSet};

mod part1;
mod part2;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

fn dfs(point: (i32, i32), point_set: &mut HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    if !point_set.contains(&point) {
        return HashSet::new();
    }
    point_set.remove(&point);
    let mut new_group = HashSet::new();
    new_group.insert(point);
    for (i, j) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let new_point = (point.0 + i, point.1 + j);
        new_group.extend(dfs(new_point, point_set));
    }
    new_group
}

fn parse(input: &str) -> HashMap<char, HashSet<(i32, i32)>> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut info: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c_set = &mut info.entry(grid[i][j]).or_insert(HashSet::new());
            c_set.insert((i.try_into().unwrap(), j.try_into().unwrap()));
        }
    }
    info
}

pub fn run_p1() {
    assert_eq!(part1::solve(EXAMPLE), 1930);
    let result = part1::solve(INPUT);
    println!("Day 12, Part 1: {}", result);
}

pub fn run_p2() {
    assert_eq!(part2::solve(include_str!("ex2")), 368);
    assert_eq!(part2::solve(EXAMPLE), 1206);
    let result = part2::solve(INPUT);
    println!("Day 11, Part 2: {}", result);
}
