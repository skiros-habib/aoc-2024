use std::collections::BTreeSet;

use super::parse;
use itertools::Itertools;

fn limit_add(a: usize, b: usize, limit: usize) -> Option<usize> {
    let result = a + b;
    if result > limit {
        None
    } else {
        Some(result)
    }
}

fn get_antinodes(
    first: (usize, usize),
    second: (usize, usize),
    bounds: (usize, usize),
) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let (y1, x1) = first;
    let (y2, x2) = second;
    let dx = x2 as i32 - x1 as i32;
    let dy = y2 - y1;
    let anti_node_upper = y1.checked_sub(dy);
    let anti_node_lower = limit_add(y2, dy, bounds.0);
    if x1 == x2 {
        return (anti_node_upper.zip(Some(x1)), anti_node_lower.zip(Some(x1)));
    }
    match dx > 0 {
        true => (
            anti_node_lower.zip(limit_add(x2, dx as usize, bounds.1)),
            anti_node_upper.zip(x1.checked_sub(dx as usize)),
        ),
        false => (
            anti_node_lower.zip(x2.checked_sub(dx.abs() as usize)),
            anti_node_upper.zip(limit_add(x1, dx.abs() as usize, bounds.1)),
        ),
    }
}

pub fn solve(input: &str) -> usize {
    let (info, bounds) = parse(input);
    let mut antinodes: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (c, coords) in info.iter() {
        // println!("{} {:?}", c, coords);
        for (first, second) in coords.iter().tuple_combinations() {
            let antinode = get_antinodes(*first, *second, bounds);
            if antinode.0.is_some() {
                antinodes.insert(antinode.0.unwrap());
            }
            if antinode.1.is_some() {
                antinodes.insert(antinode.1.unwrap());
            }
        }
    }
    for i in &antinodes {
        println!("{:?}", i);
    }
    antinodes.len()
}
