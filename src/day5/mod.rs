mod part1;
mod part2;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

use std::collections::{BTreeMap, BTreeSet};

fn parse_input(input: &str) -> (BTreeMap<u32, BTreeSet<u32>>, Vec<Vec<u32>>) {
    let mut split = input.split("\n\n");
    let mut rules: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    split
        .next()
        .expect("There should be a first chunk after seperation by \\n\\n")
        .lines()
        .map(|line| {
            line.split_once('|')
                .expect("Each line in the rules should be seperated by |")
        })
        .map(|(a, b)| {
            (
                a.parse().expect("Each rule should contain two numbers"),
                b.parse().expect("Each rule should contain two numbers"),
            )
        })
        .for_each(|(a, b)| {
            rules.entry(b).or_default().insert(a);
        });
    let updates: Vec<Vec<u32>> = split
        .next()
        .expect("There should be a second chunk after seperation by \\n\\n")
        .lines()
        .map(|line| {
            line.split(',')
                .map(|i| {
                    i.parse()
                        .expect("Each update should contain numbers seperated by ,")
                })
                .collect()
        })
        .collect();
    (rules, updates)
}

pub fn run_p1() {
    assert_eq!(part1::solve(EXAMPLE), 143);
    let result = part1::solve(INPUT);
    println!("Day 5, Part 1: {}", result);
}

pub fn run_p2() {
    assert_eq!(part2::solve(EXAMPLE), 123);
    let result = part2::solve(INPUT);
    println!("Day 5, Part 2: {}", result);
}
