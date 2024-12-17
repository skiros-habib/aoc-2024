use std::collections::{HashMap, HashSet};

use super::parse_grid;

pub fn solve(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut start = (0, 0);
    'outer: for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                start = (i as i32, j as i32);
                break 'outer;
            }
        }
    }
    let mut obstacles: Vec<(i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &c)| {
                if c == '#' {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .collect();
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' {
                obstacles.push((i as i32, j as i32));
                // println!("{:?} {:?} ", i, j);
                sum += search_grid(start, &obstacles) as usize;
                obstacles.pop();
            }
        }
    }

    sum
}

fn get_next_closest_obstacle(
    current: (i32, i32),
    obstacles: &Vec<(i32, i32)>,
    direction: char,
) -> Option<&(i32, i32)> {
    match direction {
        '^' => obstacles
            .iter()
            .filter(|(i, j)| *i < current.0 && *j == current.1)
            .min_by_key(|(i, _)| current.0 - i),
        'v' => obstacles
            .iter()
            .filter(|(i, j)| *i > current.0 && *j == current.1)
            .min_by_key(|(i, _)| i - current.0),
        '<' => obstacles
            .iter()
            .filter(|(i, j)| *i == current.0 && *j < current.1)
            .min_by_key(|(_, j)| current.1 - j),
        '>' => obstacles
            .iter()
            .filter(|(i, j)| *i == current.0 && *j > current.1)
            .min_by_key(|(_, j)| j - current.1),
        _ => panic!("Invalid direction"),
    }
}

fn search_grid(start: (i32, i32), obstacles: &Vec<(i32, i32)>) -> bool {
    let mut current = start;
    let added_obstacle = obstacles
        .last()
        .expect("There should be at least one obstacle")
        .to_owned();
    let mut direction_counter: HashMap<char, usize> = HashMap::new();
    let mut last_dir = '^';
    let mut visited_set: HashSet<(i32, i32)> = HashSet::new();
    let mut visited: Vec<(i32, i32)> = Vec::new();
    loop {
        let next_obstacle = get_next_closest_obstacle(current, obstacles, last_dir);
        if let Some((i, j)) = next_obstacle {
            if (*i, *j) == added_obstacle {
                let count = direction_counter.entry(last_dir).or_insert(0);
                *count += 1;
            }
            if direction_counter.iter().any(|(_, &count)| count > 1) {
                return true;
            }
            if direction_counter.iter().any(|(_, &count)| count == 1) {
                visited.push(current);
                let res = visited_set.insert(current);
                if !res && visited.first().unwrap() == &current && visited.len() > 2 {
                    return true;
                } else if !res {
                    visited.clear();
                    visited_set.clear();
                }
            }
            match last_dir {
                '^' => {
                    current = (*i + 1, *j);
                    last_dir = '>';
                }
                'v' => {
                    current = (*i - 1, *j);
                    last_dir = '<';
                }
                '<' => {
                    current = (*i, *j + 1);
                    last_dir = '^';
                }
                '>' => {
                    current = (*i, *j - 1);
                    last_dir = 'v';
                }
                _ => panic!("Invalid direction"),
            };
        } else {
            break;
        }
    }
    false
}
