use super::{dfs, parse};

pub fn solve(input: &str) -> usize {
    let info = parse(input);
    let mut total = 0;
    for (_p, v) in info {
        let mut point_set = v.clone();
        while !point_set.is_empty() {
            if let Some(&point) = point_set.iter().next() {
                let group = dfs(point, &mut point_set);
                let area = group.len();
                let perimeter = group.iter().fold(0, |acc, (i, j)| {
                    acc + vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
                        .iter()
                        .map(|(x, y)| (i + x, j + y))
                        .filter(|p| !group.contains(p))
                        .count()
                });
                total += area * perimeter;
            }
        }
    }
    total
}
