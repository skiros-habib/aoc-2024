use super::parse_grid;

pub fn solve(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut total = 0;
    for i in 0..grid.len() - 2 {
        for j in 0..grid[0].len() - 2 {
            let middle = grid[i + 1].chars().nth(j + 1).unwrap();
            if middle != 'A' {
                continue;
            }
            let a = grid[i].bytes().nth(j).unwrap() as u16;
            let b = grid[i].bytes().nth(j + 2).unwrap() as u16;
            let c = grid[i + 2].bytes().nth(j).unwrap() as u16;
            let d = grid[i + 2].bytes().nth(j + 2).unwrap() as u16;
            if a + b + c + d == 320 && a != d && b != c {
                total += 1;
            }
        }
    }
    total
}
