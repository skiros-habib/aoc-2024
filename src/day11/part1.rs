use super::observe;
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let input = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut stones = HashMap::new();
    input.iter().for_each(|&stone| {
        stones.entry(stone).and_modify(|e| *e += 1).or_insert(1);
    });
    for _ in 0..25 {
        stones = observe(stones);
    }
    stones.values().sum()
}
