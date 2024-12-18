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
) -> BTreeSet<(usize, usize)> {
    let mut result = BTreeSet::new();
    let (x1, y1) = first;
    let (x2, y2) = second;
    let dx = x2 - x1;
    let dy = y2 as isize - y1 as isize;
    let mut test_x = Some(x1);
    let mut test_y = Some(y1);
    while let Some((x, y)) = check_bounds(test_x.zip(test_y), bounds) {
        result.insert((x, y));
        test_x = x.checked_sub(dx);
        test_y = usize::try_from((y as isize) - dy).ok();
    }
    let mut test_x = Some(x2);
    let mut test_y = Some(y2);
    while let Some((x, y)) = check_bounds(test_x.zip(test_y), bounds) {
        result.insert((x, y));
        test_x = Some(x + dx);
        test_y = y.checked_add_signed(dy);
    }
    result
}

pub fn solve(input: &str) -> usize {
    let (info, bounds) = parse(input);
    let mut all_antinodes: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (_c, coords) in info.iter() {
        for (first, second) in coords.iter().tuple_combinations() {
            let antinodes = get_antinodes(*first, *second, bounds);
            all_antinodes.extend(antinodes);
        }
    }
    all_antinodes.len()
}
