use std::collections::VecDeque;

use super::parse_grid;

fn print_grid(grid: Vec<Vec<char>>) {
    for i in grid {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}

pub fn solve(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' {
                let mut new_grid = grid.clone();
                new_grid[i][j] = '#';
                if attempt_solve(new_grid) {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn attempt_solve(mut grid: Vec<Vec<char>>) -> bool {
    let mut obstacles: VecDeque<(i32, i32)> = VecDeque::with_capacity(5);
    let mut obs_tracker: Vec<(i32, i32)> = Vec::new();
    let mut last_pos: (i32, i32) = (0, 0);
    let mut last_dir = '^';
    'outer: for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                last_pos = (i as i32, j as i32);
                break 'outer;
            }
        }
    }
    loop {
        let mut next_pos_vec: (i32, i32) = match last_dir {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!("Invalid direction"),
        };
        let mut next_dir = last_dir;
        let mut next_pos = ((last_pos.0 + next_pos_vec.0), (last_pos.1 + next_pos_vec.1));
        if next_pos.0 as usize >= grid.len() || next_pos.1 as usize >= grid[0].len() {
            break;
        }
        if next_pos.0 < 0 || next_pos.1 < 0 {
            break;
        }
        if grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            obstacles.push_back(next_pos);
            obs_tracker.push(next_pos);
            if obstacles.len() == 6 {
                obstacles.pop_front();
                // println!(
                //     "{:?}\n{:?}",
                //     obs_tracker.get(obs_tracker.len() - 5..).unwrap(),
                //     obstacles
                // );
            }

            if obstacles.len() == 5 {
                // println!(
                //     "{:?} {:?}",
                //     obstacles.get(0).unwrap(),
                //     obstacles.get(4).unwrap()
                // );
                if obstacles.get(0).unwrap() == obstacles.get(4).unwrap() {
                    return true;
                }
            }
            next_pos_vec = match last_dir {
                '>' => (1, 0),
                '<' => (-1, 0),
                'v' => (0, -1),
                '^' => (0, 1),
                _ => panic!("Invalid direction"),
            };
            next_pos = ((last_pos.0 + next_pos_vec.0), (last_pos.1 + next_pos_vec.1));
            next_dir = match last_dir {
                '>' => 'v',
                '<' => '^',
                'v' => '<',
                '^' => '>',
                _ => panic!("Invalid direction"),
            };
        }
        last_pos = next_pos;
        last_dir = next_dir;
    }
    false
}
