use std::collections::BTreeSet;

use super::parse;
use itertools::Itertools;

fn check_bounds(point: Option<(usize, usize)>, bounds: (usize, usize)) -> Option<(usize, usize)> {
    match point {
        Some((x, y)) => {
            if x < bounds.0 && y < bounds.1 {
                Some((x, y))
            } else {
                None
            }
        }
        None => None,
    }
}

fn get_antinodes(
    first: (usize, usize),
    second: (usize, usize),
    bounds: (usize, usize),
) -> [Option<(usize, usize)>; 2] {
    let (x1, y1) = first;
    let (x2, y2) = second;
    let dx = x2 - x1;
    let dy = y2 as isize - y1 as isize;
    [
        check_bounds(
            x1.checked_sub(dx)
                .zip(usize::try_from((y1 as isize) - dy).ok()),
            bounds,
        ),
        check_bounds(Some(x2 + dx).zip(y2.checked_add_signed(dy)), bounds),
    ]
}

pub fn solve(input: &str) -> usize {
    let (info, bounds) = parse(input);
    let mut all_antinodes: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (_c, coords) in info.iter() {
        for (first, second) in coords.iter().tuple_combinations() {
            let antinodes = get_antinodes(*first, *second, bounds);
            antinodes.iter().for_each(|antinode| match antinode {
                Some(antinode) => {
                    all_antinodes.insert(*antinode);
                }
                None => {}
            });
        }
    }
    all_antinodes.len()
}
