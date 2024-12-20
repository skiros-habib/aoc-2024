use std::collections::HashSet;

use super::{dfs, parse};

// let perimeter = group.iter().fold(0, |acc, (i, j)| {
//     acc + vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
//         .iter()
//         .map(|(x, y)| (i + x, j + y))
//         .filter(|p| !group.contains(p))
//         .count()
// });

fn count_perimeter(points: &HashSet<(i32, i32)>) -> usize {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut all = HashSet::new();
    let mut total = 0;

    for &point in points {
        let edges: HashSet<(i32, i32)> = directions
            .iter()
            .filter_map(|&(dx, dy)| {
                let neighbor = (point.0 + dx, point.1 + dy);
                if points.contains(&neighbor) {
                    None
                } else {
                    Some(neighbor)
                }
            })
            .collect();
        all.extend(edges);
    }
    while !all.is_empty() {
        if let Some(&point) = all.iter().next() {
            let group = dfs(point, &mut all);
            println!("{:?}", group);
            if group.len() > 0 {
                total += 1; // need to actually check this...
            }
        }
    }
    total
}

pub fn solve(input: &str) -> usize {
    let info = parse(input);
    let mut total = 0;
    for (_p, v) in info {
        println!("{_p}");
        let mut point_set = v.clone();
        while !point_set.is_empty() {
            if let Some(&point) = point_set.iter().next() {
                let group = dfs(point, &mut point_set);
                let area = group.len();
                let edges = count_perimeter(&group);
                println!("");
                // println!("{}:\t{}\t{}\t{}", _p, area, edges, area * edges);
                total += area * edges;
            }
        }
    }
    total
}
