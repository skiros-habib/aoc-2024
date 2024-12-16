use super::parse_grid;

pub fn solve(input: &str) -> usize {
    let mut grid = parse_grid(input);
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
        grid[last_pos.0 as usize][last_pos.1 as usize] = 'X';
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
        grid[next_pos.0 as usize][next_pos.1 as usize] = next_dir;
        last_pos = next_pos;
        last_dir = next_dir;
    }
    grid.concat().iter().filter(|&x| *x == 'X').count()
}
